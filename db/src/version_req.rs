use nom::{digit, is_space};
use std::str::{self, FromStr};
use serde::de::{Deserialize, Deserializer, Error, Unexpected};
use Version;

#[derive(Clone, Debug)]
pub enum VersionReq {
    Bracket(Box<VersionReq>),
    And(Box<VersionReq>, Box<VersionReq>),
    Or(Box<VersionReq>, Box<VersionReq>),
    Eq(Version),
    Ge(Version),
    Le(Version),
    Gt(Version),
    Lt(Version),
}

impl<'a> Deserialize<'a> for VersionReq {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).and_then(|v|
            parse_version_req(v.as_bytes())
                .to_result()
                .map_err(|e| D::Error::invalid_value(Unexpected::Other(&e.to_string()), &"version requirement"))
        )
    }
}

impl VersionReq {
    pub fn matches(&self, v: &Version) -> bool {
        match *self {
            VersionReq::Bracket(ref r) => r.matches(v),
            VersionReq::And(ref lhs, ref rhs) => lhs.matches(v) && rhs.matches(v),
            VersionReq::Or(ref lhs, ref rhs) => lhs.matches(v) || rhs.matches(v),
            VersionReq::Eq(ref version) => v == version,
            VersionReq::Ge(ref version) => v >= version,
            VersionReq::Le(ref version) => v <= version,
            VersionReq::Lt(ref version) => v < version,
            VersionReq::Gt(ref version) => v > version,
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
           major: major,
           minor: minor,
        })
    )
);

named!(parse_version_req<&[u8], VersionReq>,
    alt!(
        do_parse!(
            tag!("<=") >>
            v: parse_version >>
            (VersionReq::Le(v))
        ) |
        do_parse!(
            tag!("<") >>
            v: parse_version >>
            (VersionReq::Lt(v))
        ) |
        do_parse!(
            tag!(">=") >>
            v: parse_version >>
            (VersionReq::Ge(v))
        ) |
        do_parse!(
            tag!(">") >>
            v: parse_version >>
            (VersionReq::Gt(v))
        ) |
        do_parse!(
            tag!("=") >>
            v: parse_version >>
            (VersionReq::Eq(v))
        ) |
        map!(parse_version, VersionReq::Eq) |
        do_parse!(
            lhs: parse_version_req >>
            tag!("&&") >>
            rhs: parse_version_req >>
            (VersionReq::And(Box::new(lhs), Box::new(rhs)))
        ) |
        do_parse!(
            lhs: parse_version_req >>
            tag!("||") >>
            rhs: parse_version_req >>
            (VersionReq::Or(Box::new(lhs), Box::new(rhs)))
        ) |
        map!(
            delimited!(
                char!('('),
                parse_version_req,
                char!(')')
            ),
            |v| VersionReq::Bracket(Box::new(v))
        )
    )
);
