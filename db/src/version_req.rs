use {
    mccs::Version,
    nom::{digit, is_hex_digit, is_space},
    serde::{
        de::{Deserialize, Deserializer, Error, Unexpected},
        ser::{Serialize, Serializer},
    },
    std::{
        cmp, fmt,
        str::{self, FromStr},
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Req<V> {
    Bracket(Box<Req<V>>),
    And(Box<Req<V>>, Box<Req<V>>),
    Or(Box<Req<V>>, Box<Req<V>>),
    Eq(V),
    Ge(V),
    Le(V),
    Gt(V),
    Lt(V),
}

impl<V: fmt::Display> fmt::Display for Req<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Req::Bracket(r) => write!(f, "({r})"),
            Req::And(lhs, rhs) => write!(f, "{lhs} && {rhs}"),
            Req::Or(lhs, rhs) => write!(f, "{lhs} || {rhs}"),
            Req::Eq(v) => write!(f, "={v}"),
            Req::Gt(v) => write!(f, ">{v}"),
            Req::Ge(v) => write!(f, ">={v}"),
            Req::Lt(v) => write!(f, "<{v}"),
            Req::Le(v) => write!(f, "<={v}"),
        }
    }
}

pub(crate) type VersionReq = Req<Version>;

trait ReqValue: Sized + fmt::Debug + fmt::Display {
    type Err: fmt::Debug;
    const EXPECTED: &'static str;

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err>;
}

impl ReqValue for Version {
    type Err = nom::ErrorKind;

    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        complete!(req.as_bytes(), parse_version_expr)
            .to_result()
            .map_err(|e| e.into_error_kind())
    }
}

impl ReqValue for u8 {
    type Err = nom::ErrorKind;

    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        complete!(req.as_bytes(), parse_u8_expr)
            .to_result()
            .map_err(|e| e.into_error_kind())
    }
}

impl<'a, V: ReqValue> Deserialize<'a> for Req<V> {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).and_then(|v| {
            V::parse_req(&v).map_err(|e| D::Error::invalid_value(Unexpected::Other(&format!("{:?}", e)), &V::EXPECTED))
        })
    }
}

impl<V: ReqValue> Serialize for Req<V> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(s)
    }
}

impl<V> Req<V> {
    #[cfg(test)]
    fn and<R: Into<Box<Self>>>(self, rhs: R) -> Self {
        Req::And(self.into(), rhs.into())
    }

    #[cfg(test)]
    fn or<R: Into<Box<Self>>>(self, rhs: R) -> Self {
        Req::Or(self.into(), rhs.into())
    }
}

impl<V> Req<V>
where
    V: cmp::PartialEq + cmp::PartialOrd,
{
    pub fn matches(&self, v: &V) -> bool {
        match *self {
            Req::Bracket(ref r) => r.matches(v),
            Req::And(ref lhs, ref rhs) => lhs.matches(v) && rhs.matches(v),
            Req::Or(ref lhs, ref rhs) => lhs.matches(v) || rhs.matches(v),
            Req::Eq(ref req) => v == req,
            Req::Ge(ref req) => v >= req,
            Req::Le(ref req) => v <= req,
            Req::Lt(ref req) => v < req,
            Req::Gt(ref req) => v > req,
        }
    }
}

impl Default for VersionReq {
    fn default() -> Self {
        VersionReq::Ge(Version::new(0, 0))
    }
}

named!(parse_version<&[u8], Version>,
    do_parse!(
        take_while!(is_space) >>
        major: map_res!(digit, |v| u8::from_str(str::from_utf8(v).unwrap())) >>
        tag!(".") >>
        minor: map_res!(digit, |v| u8::from_str(str::from_utf8(v).unwrap())) >>
        take_while!(is_space) >>
        (Version {
            major,
            minor,
        })
    )
);

named!(parse_u8<&[u8], u8>,
    do_parse!(
        take_while!(is_space) >>
        tag!("0x") >>
        req: map_res!(take_while!(is_hex_digit), |v| u8::from_str_radix(str::from_utf8(v).unwrap(), 16)) >>
        take_while!(is_space) >>
        (req)
    )
);

named!(parse_version_expr<&[u8], VersionReq>,
    do_parse!(
        lhs: parse_version_req >>
        res: opt!(alt!(
            do_parse!(
                complete!(tag!("&&")) >>
                rhs: parse_version_req >>
                (Req::And as fn(_, _) -> VersionReq, Box::new(rhs))
            ) |
            do_parse!(
                complete!(tag!("||")) >>
                rhs: parse_version_req >>
                (Req::Or as fn(_, _) -> VersionReq, Box::new(rhs))
            )
        )) >>
        (match res {
            Some((op, rhs)) => op(Box::new(lhs), rhs),
            None => lhs,
        })
    )
);

named!(parse_version_req<&[u8], VersionReq>,
    alt!(
        do_parse!(
            complete!(tag!("<=")) >>
            v: parse_version >>
            (Req::Le(v))
        ) |
        do_parse!(
            complete!(tag!("<")) >>
            v: parse_version >>
            (Req::Lt(v))
        ) |
        do_parse!(
            complete!(tag!(">=")) >>
            v: parse_version >>
            (Req::Ge(v))
        ) |
        do_parse!(
            complete!(tag!(">")) >>
            v: parse_version >>
            (Req::Gt(v))
        ) |
        do_parse!(
            complete!(tag!("=")) >>
            v: parse_version >>
            (Req::Eq(v))
        ) |
        map!(parse_version, Req::Eq)
    )
);

named!(parse_u8_expr<&[u8], Req<u8>>,
    do_parse!(
        lhs: alt!(
            map!(
                delimited!(
                    char!('('),
                    parse_u8_expr,
                    char!(')')
                ),
                |v| Req::Bracket(Box::new(v))
            ) |
            parse_u8_req
        ) >>
        res: opt!(alt!(
            do_parse!(
                take_while!(is_space) >>
                complete!(tag!("&&")) >>
                take_while!(is_space) >>
                rhs: parse_u8_expr >>
                (Req::And as fn(_, _) -> Req<u8>, Box::new(rhs))
            ) |
            do_parse!(
                take_while!(is_space) >>
                complete!(tag!("||")) >>
                take_while!(is_space) >>
                rhs: parse_u8_expr >>
                (Req::Or as fn(_, _) -> Req<u8>, Box::new(rhs))
            )
        )) >>
        (match res {
            Some((op, rhs)) => op(Box::new(lhs), rhs),
            None => lhs,
        })
    )
);

named!(parse_u8_req<&[u8], Req<u8>>,
    alt!(
        do_parse!(
            complete!(tag!("<=")) >>
            v: parse_u8 >>
            (Req::Le(v))
        ) |
        do_parse!(
            complete!(tag!("<")) >>
            v: parse_u8 >>
            (Req::Lt(v))
        ) |
        do_parse!(
            complete!(tag!(">=")) >>
            v: parse_u8 >>
            (Req::Ge(v))
        ) |
        do_parse!(
            complete!(tag!(">")) >>
            v: parse_u8 >>
            (Req::Gt(v))
        ) |
        do_parse!(
            complete!(tag!("=")) >>
            v: parse_u8 >>
            (Req::Eq(v))
        ) |
        map!(parse_u8, Req::Eq)
    )
);

#[test]
fn version() {
    assert_eq!(parse_version(b"22.01").to_result().unwrap(), Version {
        major: 22,
        minor: 1
    })
}

#[test]
fn version_eq() {
    assert_eq!(
        parse_version_expr(b"2.0").to_result().unwrap(),
        VersionReq::Eq(Version { major: 2, minor: 0 })
    )
}

#[test]
fn version_req() {
    assert_eq!(
        parse_version_expr(b"<=3.1").to_result().unwrap(),
        VersionReq::Le(Version { major: 3, minor: 1 })
    )
}

#[test]
fn u8_req() {
    assert_eq!(parse_u8_expr(b"<=0x0a").to_result().unwrap(), Req::Le(0xa))
}

#[test]
fn u8_expr() {
    assert_eq!(
        parse_u8_expr(b"0x0a && <0x05").to_result().unwrap(),
        Req::Eq(0xa).and(Req::Lt(5))
    )
}

#[test]
fn u8_chained_expr() {
    assert_eq!(
        parse_u8_expr(b"(0x0a && <0x05) || (0x01 && 0x02)").to_result().unwrap(),
        Req::Bracket(Req::Eq(0xa).and(Req::Lt(5)).into()).or(Req::Bracket(Req::Eq(1).and(Req::Eq(2)).into()))
    )
}

#[test]
fn u8_ordered_chain() {
    assert_eq!(
        parse_u8_expr(b"0x0a || 0x01 || 0x02").to_result().unwrap(),
        Req::Eq(0xa).or(Req::Eq(1).or(Req::Eq(2)))
    )
}

#[test]
fn u8_nested_expr() {
    assert_eq!(
        parse_u8_expr(b"((0x0a && (0x01 && 0x02)) || <0x03)")
            .to_result()
            .unwrap(),
        Req::Bracket(
            Req::Bracket(Req::Eq(0xa).and(Req::Bracket(Req::Eq(1).and(Req::Eq(2)).into())).into())
                .or(Req::Lt(3))
                .into()
        )
    )
}

#[test]
fn u8_input_select_expr() {
    assert_eq!(
        parse_u8_expr(b"(>=0x13 && <=0x18) || =0x1A || >=0x1C")
            .to_result()
            .unwrap(),
        Req::Bracket(Req::Ge(0x13).and(Req::Le(0x18)).into()).or(Req::Eq(0x1a).or(Req::Ge(0x1c)))
    )
}
