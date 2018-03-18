#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/mccs-rs/")]

//! VESA Monitor Command Control Set standardizes the meaning of DDC/CI VCP
//! feature codes, and allows a display to broadcast its capabilities to the
//! host.

#[macro_use]
extern crate nom;
#[cfg(feature = "void")]
extern crate void;

use std::io;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::collections::{BTreeMap, btree_map};

#[cfg(feature = "void")]
use void::Void;
#[cfg(not(feature = "void"))]
pub enum Void { }

mod parse;
#[cfg(test)]
mod testdata;

/// VCP feature code
pub type FeatureCode = u8;

/// Extended Display Identification Data
pub type EdidData = Vec<u8>;

/// Video Display Information Format
pub type VdifData = Vec<u8>;

/// Parsed display capabilities string.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capabilities {
    /// The protocol class.
    ///
    /// It's not very clear what this field is for.
    pub protocol: Option<Protocol>,
    /// The type of display.
    pub ty: Option<Type>,
    /// The model name/number of the display.
    pub model: Option<String>,
    /// A list of the supported VCP commands.
    pub commands: Vec<u8>,
    /// A value of `1` seems to indicate that the monitor has passed Microsoft's
    /// Windows Hardware Quality Labs testing.
    pub ms_whql: Option<u8>,
    /// Monitor Command Control Set version code.
    pub mccs_version: Option<Version>,
    /// Virtual Control Panel feature code descriptors.
    pub vcp_features: BTreeMap<FeatureCode, VcpDescriptor>,
    /// Extended Display Identification Data
    ///
    /// Note that although the standard defines this field, in practice it
    /// is not used and instead the EDID is read from a separate I2C EEPROM on
    /// the monitor.
    pub edid: Option<EdidData>,
    /// Video Display Information Format are optional extension blocks for the
    /// EDID. Like the EDID field, this is probably not in use.
    pub vdif: Vec<VdifData>,
    /// Additional unrecognized data from the capability string.
    pub unknown_tags: Vec<UnknownTag>,
}

/// Display protocol class
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    /// Standard monitor
    Monitor,
    /// I have never seen this outside of an MCCS spec example, it may be a typo.
    Display,
    /// Unrecognized protocol class
    Unknown(String),
}

impl<'a> From<&'a str> for Protocol {
    fn from(s: &'a str) -> Self {
        match s {
            "monitor" => Protocol::Monitor,
            "display" => Protocol::Display,
            s => Protocol::Unknown(s.into()),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(match *self {
            Protocol::Monitor => "monitor",
            Protocol::Display => "display",
            Protocol::Unknown(ref s) => s,
        }, f)
    }
}

impl FromStr for Protocol {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

/// Display type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// Cathode Ray Tube display
    Crt,
    /// Liquid Crystal Display
    Lcd,
    /// Also an LCD, I'm not sure this should exist.
    Led,
    /// Unrecognized display type
    Unknown(String),
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            s if s.eq_ignore_ascii_case("crt") => Type::Crt,
            s if s.eq_ignore_ascii_case("lcd") => Type::Lcd,
            s if s.eq_ignore_ascii_case("led") => Type::Led,
            s => Type::Unknown(s.into()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(match *self {
            Type::Crt => "crt",
            Type::Lcd => "lcd",
            Type::Led => "led",
            Type::Unknown(ref s) => s,
        }, f)
    }
}

impl FromStr for Type {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

/// Monitor Command Control Set specification version code
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// Major version number
    pub major: u8,
    /// Minor revision version
    pub minor: u8,
}

/// Descriptive information about a supported VCP feature code.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VcpDescriptor {
    /// The name of the feature code, if different from the standard MCCS spec.
    pub name: Option<String>,
    /// Allowed values for this feature, and optionally their names.
    ///
    /// This is used for non-continuous VCP types.
    pub values: BTreeMap<u8, Option<String>>,
}

impl VcpDescriptor {
    /// The allowed values for this feature code.
    pub fn values(&self) -> btree_map::Keys<u8, Option<String>> {
        self.values.keys()
    }
}

/// An unrecognized entry in the capability string
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownTag {
    /// The name of the entry
    pub name: String,
    /// The data contained in the entry, usually an unparsed string.
    pub data: UnknownData,
}

/// Data that can be contained in a capability entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnknownData {
    /// UTF-8/ASCII data
    String(String),
    /// Data that is not valid UTF-8
    StringBytes(Vec<u8>),
    /// Length-prefixed binary data
    Binary(Vec<u8>),
}

/// Parses a MCCS capability string.
pub fn parse_capabilities<C: AsRef<[u8]>>(capability_string: C) -> io::Result<Capabilities> {
    use parse::Cap;

    parse::parse_capabilities(capability_string.as_ref()).map(|c| {
        // TODO: check for multiple tags of anything only allowed once?

        let mut caps = Capabilities::default();
        for cap in &c {
            match *cap {
                Cap::Protocol(protocol) => caps.protocol = Some(protocol.into()),
                Cap::Type(ty) => caps.ty = Some(ty.into()),
                Cap::Model(model) => caps.model = Some(model.into()),
                Cap::Commands(ref cmds) => caps.commands = cmds.clone(),
                Cap::Whql(whql) => caps.ms_whql = Some(whql),
                Cap::MccsVersion(major, minor) => caps.mccs_version = Some(Version {
                    major: major,
                    minor: minor,
                }),
                Cap::Vcp(ref vcp) => for &(code, ref values) in vcp {
                    caps.vcp_features.entry(code).or_insert_with(|| VcpDescriptor::default())
                        .values.extend(values.iter().flat_map(|i| i).map(|&v| (v, None)))
                },
                Cap::VcpNames(..) => (), // wait until after processing vcp() section
                Cap::Unknown(name, value) => caps.unknown_tags.push(UnknownTag {
                    name: name.into(),
                    data: UnknownData::String(value.into()),
                }),
                Cap::UnknownBytes(name, value) => caps.unknown_tags.push(UnknownTag {
                    name: name.into(),
                    data: UnknownData::StringBytes(value.into()),
                }),
                Cap::UnknownBinary(name, value) => caps.unknown_tags.push(UnknownTag {
                    name: name.into(),
                    data: UnknownData::Binary(value.into()),
                }),
                Cap::Edid(edid) => caps.edid = Some(edid.into()),
                Cap::Vdif(vdif) => caps.vdif.push(vdif.into()),
            }
        }

        for cap in c {
            match cap {
                Cap::VcpNames(vcp) => for (code, name, value_names) in vcp {
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
                },
                _ => (),
            }
        }

        caps
    })
}

#[test]
fn capability_string_samples() {
    for sample in ::testdata::test_data() {
        let caps = parse_capabilities(sample).expect("Failed to parse capabilities");
        println!("Caps: {:#?}", caps);
    }
}
