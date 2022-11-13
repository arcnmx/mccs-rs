#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/mccs-caps/0.2.0")]

//! MCCS compliant displays will report their supported capabilities in a string
//! retrieved over DDC/CI. The format of this string is specified in the DDC
//! specification, MCCS, and ACCESS.bus section 7. This crate parses the
//! capability string into structured data.

pub use self::{
    caps::{Cap, Vcp, VcpName, VcpValue},
    entries::ValueParser,
};
use {
    mccs::{Capabilities, UnknownData, UnknownTag, VcpDescriptor, Version},
    nom::Finish,
    std::{fmt, io, str},
};

#[cfg(test)]
mod testdata;

#[allow(missing_docs)]
mod caps;
#[allow(missing_docs)]
mod entries;

/// Parses a MCCS capability string.
pub fn parse_capabilities<C: AsRef<[u8]>>(capability_string: C) -> io::Result<Capabilities> {
    let capability_string = capability_string.as_ref();
    let entries = Value::parse_capabilities(capability_string);

    // TODO: check for multiple tags of anything only allowed once?

    let mut caps = Capabilities::default();
    let mut vcpnames = Vec::new();
    for cap in Cap::parse_entries(entries) {
        match cap? {
            Cap::Protocol(protocol) => caps.protocol = Some(protocol.into()),
            Cap::Type(ty) => caps.ty = Some(ty.into()),
            Cap::Model(model) => caps.model = Some(model.into()),
            Cap::Commands(ref cmds) => caps.commands = cmds.clone(),
            Cap::Whql(whql) => caps.ms_whql = Some(whql),
            Cap::MccsVersion(major, minor) => caps.mccs_version = Some(Version::new(major, minor)),
            Cap::Vcp(ref vcp) =>
                for Vcp {
                    feature: code,
                    ref values,
                } in vcp
                {
                    caps.vcp_features
                        .entry(*code)
                        .or_insert_with(|| VcpDescriptor::default())
                        .values
                        .extend(values.iter().flat_map(|i| i).map(|v| (v.value, None)))
                },
            Cap::VcpNames(v) => vcpnames.extend(v), // wait until after processing vcp() section
            Cap::Unknown(value) => caps.unknown_tags.push(UnknownTag {
                name: value.tag().into(),
                data: match value {
                    Value::String { value, .. } => match str::from_utf8(value) {
                        Ok(value) => UnknownData::String(value.into()),
                        Err(..) => UnknownData::StringBytes(value.into()),
                    },
                    Value::Binary { data, .. } => UnknownData::Binary(data.into()),
                },
            }),
            Cap::Edid(edid) => caps.edid = Some(edid.into()),
            Cap::Vdif(vdif) => caps.vdif.push(vdif.into()),
        }
    }

    for VcpName {
        feature: code,
        name,
        value_names,
    } in vcpnames
    {
        if let Some(vcp) = caps.vcp_features.get_mut(&code) {
            if let Some(name) = name {
                vcp.name = Some(name.into())
            }

            if let Some(value_names) = value_names {
                for ((_, dest), name) in vcp.values.iter_mut().zip(value_names) {
                    *dest = Some(name.into())
                }
            }
        } else {
            // TODO: should this be an error if it wasn't specified in vcp()?
        }
    }

    Ok(caps)
}

/// An entry from a capability string
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Value<'i> {
    /// A normal string
    String {
        /// The value name
        tag: &'i str,
        /// String contents
        value: &'i [u8],
    },
    /// Raw binary data
    Binary {
        /// The value name
        tag: &'i str,
        /// Data contents
        data: &'i [u8],
    },
}

impl<'i> Value<'i> {
    /// Create a new iterator over the values in a capability string
    pub fn parse_capabilities(capability_string: &'i [u8]) -> ValueParser<'i> {
        ValueParser::new(capability_string)
    }

    /// Parse a single capability string entry
    pub fn parse(data: &'i str) -> io::Result<Self> {
        Self::parse_bytes(data.as_bytes())
    }

    /// Parse a single capability string entry
    pub fn parse_bytes(data: &'i [u8]) -> io::Result<Self> {
        Self::parse_nom(data, None).finish().map(|(_, v)| v).map_err(map_err)
    }

    /// The value name
    pub fn tag(&self) -> &'i str {
        match *self {
            Value::String { tag, .. } => tag,
            Value::Binary { tag, .. } => tag,
        }
    }
}

impl From<Value<'_>> for UnknownTag {
    fn from(v: Value) -> Self {
        UnknownTag {
            name: v.tag().into(),
            data: match v {
                Value::Binary { data, .. } => UnknownData::Binary(data.into()),
                Value::String { value, .. } => match str::from_utf8(value) {
                    Ok(value) => UnknownData::String(value.into()),
                    Err(_) => UnknownData::StringBytes(value.into()),
                },
            },
        }
    }
}

impl<'a> From<&'a UnknownTag> for Value<'a> {
    fn from(v: &'a UnknownTag) -> Self {
        let tag = &v.name;
        match &v.data {
            UnknownData::Binary(data) => Value::Binary { tag, data },
            UnknownData::StringBytes(value) => Value::String { tag, value },
            UnknownData::String(value) => Value::String {
                tag,
                value: value.as_bytes(),
            },
        }
    }
}

impl<'i> fmt::Display for Value<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String { tag, value } => write!(f, "{tag}({})", value.escape_ascii()),
            Value::Binary { tag, data } => write!(f, "{tag} bin({}({}))", data.len(), data.escape_ascii()),
        }
    }
}

pub(crate) type OResult<'i, O> = Result<O, nom::error::Error<&'i [u8]>>;
pub(crate) type OResultI<'i, O> = Result<O, nom::Err<nom::error::Error<&'i [u8]>>>;

pub(crate) fn map_err(e: nom::error::Error<&[u8]>) -> io::Error {
    use nom::error::{Error, ErrorKind};

    io::Error::new(
        match e.code {
            ErrorKind::Eof | ErrorKind::Complete => io::ErrorKind::UnexpectedEof,
            _ => io::ErrorKind::InvalidData,
        },
        Error {
            input: e.input.escape_ascii().to_string(),
            code: e.code,
        },
    )
}

pub(crate) fn trim_spaces<I, O, E, P>(parser: P) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    P: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
    I: Clone + nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
{
    use nom::{character::complete::space0, sequence::delimited};

    delimited(space0, parser, space0)
}

pub(crate) fn bracketed<I, O, E, P>(parser: P) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    P: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
    I: Clone + nom::Slice<std::ops::RangeFrom<usize>> + nom::InputIter,
    <I as nom::InputIter>::Item: nom::AsChar,
{
    use nom::{character::complete::char, sequence::delimited};

    delimited(char('('), parser, char(')'))
}

#[test]
fn samples_entries() {
    for sample in testdata::test_data() {
        println!("Parsing caps: {}", String::from_utf8_lossy(sample));
        for cap in Value::parse_capabilities(sample).nom_iter() {
            println!("entry: {:?}", cap.unwrap());
        }
    }
}

#[test]
fn samples_caps() {
    for sample in testdata::test_data() {
        println!("Parsing caps: {}", String::from_utf8_lossy(sample));
        let ent = Value::parse_capabilities(sample);
        for (cap, end) in Cap::parse_entries(ent.clone()).zip(ent) {
            println!("{}", end.unwrap());
            println!("{:?}", cap.unwrap());
        }
    }
}

#[test]
fn samples_high() {
    for sample in testdata::test_data() {
        let caps = parse_capabilities(sample).expect("Failed to parse capabilities");
        println!("Caps: {:#?}", caps);
    }
}
