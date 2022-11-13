use {
    mccs::Version,
    nom::{
        branch::alt,
        bytes::complete::{tag, take_while_m_n},
        character::complete::{char, space0, u8},
        combinator::{map, map_res, opt},
        sequence::{delimited, preceded, separated_pair},
        Finish, IResult,
    },
    serde::{
        de::{Deserialize, Deserializer, Error, Unexpected},
        ser::{Serialize, Serializer},
    },
    std::{cmp, fmt},
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
    fn parse_nom(i: &str) -> IResult<&str, Self>;
}

impl ReqValue for Version {
    type Err = nom::error::ErrorKind;

    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        parse_req_expr(req).finish().map(|v| v.1).map_err(|e| e.code)
    }

    fn parse_nom(i: &str) -> IResult<&str, Self> {
        parse_version(i)
    }
}

impl ReqValue for u8 {
    type Err = nom::error::ErrorKind;

    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        parse_req_expr(req).finish().map(|v| v.1).map_err(|e| e.code)
    }

    fn parse_nom(i: &str) -> IResult<&str, Self> {
        parse_u8(i)
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

impl Default for Req<Version> {
    fn default() -> Self {
        Req::Ge(Version::new(0, 0))
    }
}

fn parse_req_expr<V: ReqValue>(i: &str) -> IResult<&str, Req<V>> {
    let (i, lhs) = match opt(delimited(char('('), parse_req_expr, char(')')))(i)? {
        (i, Some(req)) => (i, Req::Bracket(Box::new(req))),
        (_, None) => parse_req(i)?,
    };

    let (i, _) = space0(i)?;
    let tags = alt((tag("&&"), tag("||")));
    match opt(delimited(space0, tags, space0))(i)? {
        (i, Some(op)) => {
            let (i, rhs) = parse_req_expr(i)?;
            Ok((i, match op {
                "&&" => Req::And(Box::new(lhs), Box::new(rhs)),
                "||" => Req::Or(Box::new(lhs), Box::new(rhs)),
                op => unreachable!("{lhs:?} {op} {rhs:?}"),
            }))
        },
        (i, None) => Ok((i, lhs)),
    }
}

fn parse_req<V: ReqValue>(i: &str) -> IResult<&str, Req<V>> {
    let (i, _) = space0(i)?;
    let tags = alt((tag("<="), tag("<"), tag(">="), tag(">"), tag("=")));
    let op: Option<(_, fn(V) -> Req<_>)> = match opt(tags)(i)? {
        (i, Some(op)) => Some((i, match op {
            "<=" => Req::Le,
            "<" => Req::Lt,
            ">=" => Req::Ge,
            ">" => Req::Gt,
            "=" => Req::Eq,
            op => unreachable!("unknown op {op}"),
        })),
        (_, None) => None,
    };
    match op {
        Some((i, op)) => map(V::parse_nom, op)(i),
        None => map(V::parse_nom, Req::Eq)(i),
    }
}

fn hex_u8(i: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 2, |c: char| c.is_digit(16)), |i| {
        u8::from_str_radix(i, 16)
    })(i)
}

fn parse_u8(i: &str) -> IResult<&str, u8> {
    trim_spaces(alt((preceded(tag("0x"), hex_u8), u8)))(i)
}

fn parse_version(i: &str) -> IResult<&str, Version> {
    trim_spaces(map(separated_pair(u8, trim_spaces(char('.')), u8), |(major, minor)| {
        Version { major, minor }
    }))(i)
}

fn trim_spaces<I, O, E, P>(parser: P) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    P: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
    I: Clone + nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
{
    delimited(space0, parser, space0)
}

#[test]
fn version() {
    assert_eq!(parse_version("22.01").finish().unwrap().1, Version {
        major: 22,
        minor: 1
    })
}

#[test]
fn version_eq() {
    assert_eq!(
        parse_req_expr("2.0").finish().unwrap().1,
        VersionReq::Eq(Version { major: 2, minor: 0 })
    )
}

#[test]
fn version_req() {
    assert_eq!(
        parse_req_expr("<=3.1").finish().unwrap().1,
        VersionReq::Le(Version { major: 3, minor: 1 })
    )
}

#[test]
fn u8_req() {
    assert_eq!(parse_req_expr("<=0x0a").finish().unwrap().1, Req::Le(0xa))
}

#[test]
fn u8_expr() {
    assert_eq!(
        parse_req_expr("0x0a && <0x05").finish().unwrap().1,
        Req::Eq(0xa).and(Req::Lt(5))
    )
}

#[test]
fn u8_chained_expr() {
    assert_eq!(
        parse_req_expr("(0x0a && <0x05) || (0x01 && 0x02)").finish().unwrap().1,
        Req::Bracket(Req::Eq(0xa).and(Req::Lt(5)).into()).or(Req::Bracket(Req::Eq(1).and(Req::Eq(2)).into()))
    )
}

#[test]
fn u8_ordered_chain() {
    assert_eq!(
        parse_req_expr("0x0a || 0x01 || 0x02").finish().unwrap().1,
        Req::Eq(0xa).or(Req::Eq(1).or(Req::Eq(2)))
    )
}

#[test]
fn u8_nested_expr() {
    assert_eq!(
        parse_req_expr("((0x0a && (0x01 && 0x02)) || <0x03)")
            .finish()
            .unwrap()
            .1,
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
        parse_req_expr("(>=0x13 && <=0x18) || =0x1A || >=0x1C")
            .finish()
            .unwrap()
            .1,
        Req::Bracket(Req::Ge(0x13).and(Req::Le(0x18)).into()).or(Req::Eq(0x1a).or(Req::Ge(0x1c)))
    )
}
