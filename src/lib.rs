#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/mccs-rs/")]

//! VESA Monitor Command Control Set standardizes the meaning of DDC/CI VCP
//! feature codes, and allows a display to broadcast its capabilities to the
//! host.

#[cfg(feature = "void")]
extern crate void;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::collections::{BTreeMap, btree_map};

#[cfg(feature = "void")]
use void::Void;

/// An error type that cannot be encountered.
#[cfg(not(feature = "void"))]
pub enum Void { }

/// VCP feature code
pub type FeatureCode = u8;

/// VCP Value
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value {
    /// Specifies the type of the value, continuous or non-continuous.
    pub ty: u8,
    /// The high byte of the maximum allowed value.
    pub mh: u8,
    /// The low byte of the maximum allowed value.
    pub ml: u8,
    /// The high byte of the value.
    pub sh: u8,
    /// The low byte of the value.
    pub sl: u8,
}

impl Value {
    /// Create a new `Value` from a scalar value.
    ///
    /// Other fields are left as default.
    pub fn from_value(v: u16) -> Self {
        Value {
            sh: (v >> 8) as u8,
            sl: v as u8,
            .. Default::default()
        }
    }

    /// Combines the value bytes into a single value.
    pub fn value(&self) -> u16 {
        ((self.sh as u16) << 8) | self.sl as u16
    }

    /// Combines the maximum value bytes into a single value.
    pub fn maximum(&self) -> u16 {
        ((self.mh as u16) << 8) | self.ml as u16
    }

    // TODO: pub fn ty(&self) -> ValueType { } 
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Value")
            .field("maximum", &self.maximum())
            .field("value", &self.value())
            .field("ty", &self.ty)
            .finish()
    }
}

/// Extended Display Identification Data
pub type EdidData = Vec<u8>;

/// Video Display Information Format
pub type VdifData = Vec<u8>;

/// VCP feature value names
pub type ValueNames = BTreeMap<u8, Option<String>>;

// TODO: move Capabilities struct to mccs-caps? It doesn't need to be here other
// than to keep mccs-db from depending on it directly. You can't really use one
// without the other though...
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

impl Version {
    /// Create a new MCCS version from the specified version and revision.
    pub fn new(major: u8, minor: u8) -> Self {
        Version {
            major: major,
            minor: minor,
        }
    }
}

/// Descriptive information about a supported VCP feature code.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VcpDescriptor {
    /// The name of the feature code, if different from the standard MCCS spec.
    pub name: Option<String>,
    /// Allowed values for this feature, and optionally their names.
    ///
    /// This is used for non-continuous VCP types.
    pub values: ValueNames,
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
