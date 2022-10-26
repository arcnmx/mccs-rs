use nom::{digit, is_hex_digit, is_space};
use std::str::{self, FromStr};
use std::cmp;
use serde::de::{Deserialize, Deserializer, Error, Unexpected};
use serde::ser::{Serialize, Serializer};
use mccs::Version;

#[derive(Clone, Debug)]
pub enum Req<V> {
    Bracket(Box<Req<V>>),
    And(Box<Req<V>>, Box<Req<V>>),
    Or(Box<Req<V>>, Box<Req<V>>),
    Eq(V),
    Ge(V),
    Le(V),
    Gt(V),
    Lt(V),
}

pub type VersionReq = Req<Version>;

trait ReqValue: Sized {
    type Err: std::fmt::Display;
    const EXPECTED: &'static str;

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err>;
}

impl ReqValue for Version {
    type Err = nom::ErrorKind;
    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        parse_version_req(req.as_bytes()).to_result()
    }
}

impl ReqValue for u8 {
    type Err = nom::ErrorKind;
    const EXPECTED: &'static str = "version requirement";

    fn parse_req(req: &str) -> Result<Req<Self>, Self::Err> {
        parse_u8_req(req.as_bytes()).to_result()
    }
}

impl<'a, V: ReqValue> Deserialize<'a> for Req<V> {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).and_then(|v|
            V::parse_req(&v)
                .map_err(|e| D::Error::invalid_value(Unexpected::Other(&e.to_string()), &V::EXPECTED))
        )
    }
}

impl<V: ReqValue> Serialize for Req<V> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}

impl<V> Req<V> where
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

fn parse_brackets<F: FnOnce(&[u8]) -> nom::IResult<&[u8], T>, T>(input: &[u8], f: F) -> nom::IResult<&[u8], T> {
    match take_until!(input, ")") {
        nom::IResult::Done(rest, res) => match f(res) {
            nom::IResult::Done(_, res) => nom::IResult::Done(rest, res),
            nom::IResult::Error(e) => nom::IResult::Error(e),
            nom::IResult::Incomplete(needed) => todo!(),
        },
        nom::IResult::Error(e) => nom::IResult::Error(e),
        nom::IResult::Incomplete(needed) => nom::IResult::Incomplete(needed),
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
           major: major,
           minor: minor,
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

named!(parse_version_req<&[u8], VersionReq>,
    alt!(
        do_parse!(
            tag!("<=") >>
            v: parse_version >>
            (Req::Le(v))
        ) |
        do_parse!(
            tag!("<") >>
            v: parse_version >>
            (Req::Lt(v))
        ) |
        do_parse!(
            tag!(">=") >>
            v: parse_version >>
            (Req::Ge(v))
        ) |
        do_parse!(
            tag!(">") >>
            v: parse_version >>
            (Req::Gt(v))
        ) |
        do_parse!(
            tag!("=") >>
            v: parse_version >>
            (Req::Eq(v))
        ) |
        map!(parse_version, Req::Eq) |
        do_parse!(
            lhs: parse_version_req >>
            tag!("&&") >>
            rhs: parse_version_req >>
            (Req::And(Box::new(lhs), Box::new(rhs)))
        ) |
        do_parse!(
            lhs: parse_version_req >>
            tag!("||") >>
            rhs: parse_version_req >>
            (Req::Or(Box::new(lhs), Box::new(rhs)))
        )
    )
);

named!(parse_u8_req_eof<&[u8], Req<u8>>, do_parse!(
    r: parse_u8_req >>
    eof!() >>
    (r)
));
named!(parse_u8_req<&[u8], Req<u8>>,
    alt!(
        do_parse!(
            tag!("<=") >>
            v: parse_u8 >>
            eof!() >>
            (Req::Le(v))
        ) |
        do_parse!(
            tag!("<") >>
            v: parse_u8 >>
            eof!() >>
            (Req::Lt(v))
        ) |
        do_parse!(
            tag!(">=") >>
            v: parse_u8 >>
            eof!() >>
            (Req::Ge(v))
        ) |
        do_parse!(
            tag!(">") >>
            v: parse_u8 >>
            eof!() >>
            (Req::Gt(v))
        ) |
        do_parse!(
            tag!("=") >>
            v: parse_u8 >>
            eof!() >>
            (Req::Eq(v))
        ) |
        map!(parse_u8, Req::Eq) |
        map!(
            delimited!(
                char!('('),
                call!(parse_brackets, parse_u8_req_eof),
                char!(')')
            ),
            |v| Req::Bracket(Box::new(v))
        ) |
        do_parse!(
            lhs: take_until!("&&") >>
            tag!("&&") >>
            take_while!(is_space) >>
            rhs: parse_u8_req >>
            eof!() >>
            (Req::And(Box::new(parse_u8_req(lhs).unwrap().1), Box::new(rhs)))
        ) |
        do_parse!(
            lhs: take_until!("||") >>
            tag!("||") >>
            take_while!(is_space) >>
            rhs: parse_u8_req >>
            eof!() >>
            (Req::Or(Box::new(parse_u8_req(lhs).unwrap().1), Box::new(rhs)))
        )
    )
);
