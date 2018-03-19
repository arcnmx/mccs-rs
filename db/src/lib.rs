#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/mccs-rs/")]

//! Monitor Command Control Set VCP feature code meanings and data
//! interpretation.
//!
//! # Example
//!
//! ```
//! extern crate mccs_db;
//! extern crate mccs_caps;
//!
//! # fn read_display_capability_string() -> &'static str {
//! # "(prot(monitor)type(lcd)27UD58cmds(01 02 03 0C E3 F3)vcp(02 04 05 08 10 12 14(05 08 0B ) 16 18 1A 52 60( 11 12 0F 10) AC AE B2 B6 C0 C6 C8 C9 D6(01 04) DF 62 8D F4 F5(01 02) F6(00 01 02) 4D 4E 4F 15(01 06 11 13 14 28 29 32 48) F7(00 01 02 03) F8(00 01) F9 E4 E5 E6 E7 E8 E9 EA EB EF FD(00 01) FE(00 01 02) FF)mccs_ver(2.1)mswhql(1))"
//! # }
//! # fn main() {
//! // Read the capabilities from an external source, such as a monitor over DDC.
//! let caps = mccs_caps::parse_capabilities(read_display_capability_string()).unwrap();
//!
//! // Load the MCCS version spec and filter by the monitor's capabilities
//! let mut db = mccs_db::Database::from_version(caps.mccs_version.as_ref().unwrap());
//! db.apply_capabilities(&caps);
//!
//! println!("Display Capabilities: {:#?}", db);
//! # }
//! ```

#[macro_use]
extern crate nom;
extern crate mccs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[cfg(test)]
extern crate mccs_caps;

use std::collections::BTreeMap;
use std::{io, mem};
use mccs::{Capabilities, FeatureCode, ValueNames, Version, Value};

#[cfg(test)]
#[path = "../../caps/src/testdata.rs"]
mod testdata;

mod version_req;
/// Describes how to interpret a table's raw value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TableInterpretation {
    /// Generic unparsed data.
    Generic,
    /// First byte is the code page where `0x00` is the default.
    ///
    /// The range of `0xe0` to `0xff` is defined for factory use. All other
    /// values are reserved. The size of the table is unclear from the spec,
    /// maybe 4 or maybe 1?
    CodePage,
}

impl Default for TableInterpretation {
    fn default() -> Self {
        TableInterpretation::Generic
    }
}

impl TableInterpretation {
    /// Formats a table for user display.
    ///
    /// This can fail if the data is not in the expected format or has an
    /// invalid length.
    pub fn format(&self, table: &[u8]) -> Result<String, ()> {
        Ok(match *self {
            TableInterpretation::Generic => format!("{:?}", table),
            TableInterpretation::CodePage => if let Some(v) = table.get(0) {
                format!("{}", v)
            } else {
                return Err(())
            },
        })
    }
}

/// Describes how to interpret a value's raw value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueInterpretation {
    /// Generic unparsed data.
    Continuous,
    /// Generic unparsed data.
    NonContinuous,
    /// Must be set to a non-zero value in order to run the operation.
    NonZeroWrite,
    /// MCCS version is returned in `mh` (major version) and `ml` (minor/revision).
    VcpVersion,
}

impl ValueInterpretation {
    /// Formats a value for user display.
    pub fn format(&self, value: &Value) -> String {
        match *self {
            ValueInterpretation::Continuous => format!("{} / {}", value.value(), value.maximum()),
            ValueInterpretation::NonContinuous => format!("{}", value.value()),
            ValueInterpretation::NonZeroWrite => if value.sl == 0 { "unset" } else { "set" }.into(),
            ValueInterpretation::VcpVersion => format!("{}.{}", value.mh, value.ml),
        }
    }
}

/// Describes the type of a VCP value and how to interpret it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueType {
    /// The type of the data is not known
    Unknown,
    /// The data is a continuous value.
    Continuous {
        /// Describes how to interpret the continuous value.
        interpretation: ValueInterpretation,
    },
    /// The data is a non-continuous value.
    NonContinuous {
        /// The values allowed or supported to be set, as well as their
        /// user-facing names.
        values: ValueNames,
        /// Describes how to interpret the non-continuous value.
        interpretation: ValueInterpretation,
    },
    /// The data is a table (byte array)
    Table {
        /// Describes how to interpret the table.
        interpretation: TableInterpretation,
    },
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Unknown
    }
}

/// The operations allowed on a given VCP feature code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Access {
    // TODO: bitflags?
    /// The value can only be read from.
    ReadOnly,
    /// The value can only be written to.
    WriteOnly,
    /// The value is both readwritable.
    ReadWrite,
}

impl Default for Access {
    fn default() -> Self {
        Access::ReadWrite
    }
}

/// Describes a VCP feature code's functionality and value format.
#[derive(Debug, Default, Clone)]
pub struct Descriptor {
    /// The name of the feature.
    pub name: Option<String>,
    /// A detailed description of the feature.
    pub description: Option<String>,
    /// The MCCS grouping this feature belongs to.
    pub group: Option<String>,
    /// The VCP code of the feature.
    pub code: FeatureCode,
    /// The data type of the feature.
    pub ty: ValueType,
    /// Whether the feature can be set, read, or both.
    pub access: Access,
    /// Whether the feature is required to be supported by the display for MCCS
    /// specification compliance.
    pub mandatory: bool,
    /// Any other feature codes that this "interacts" with.
    ///
    /// Changing this feature's value may also affect the value of these other
    /// listed features.
    pub interacts_with: Vec<FeatureCode>,
}

/// Describes all the VCP feature codes supported by an MCCS specification or
/// display.
#[derive(Debug, Clone, Default)]
pub struct Database {
    entries: BTreeMap<FeatureCode, Descriptor>,
}

impl Database {
    fn apply_database(&mut self, db: DatabaseFile, mccs_version: &Version) -> io::Result<()> {
        for code in db.vcp_features {
            if !code.version.matches(mccs_version) {
                continue
            }

            let entry = self.entries.entry(code.code)
                .or_insert_with(|| Descriptor::default());

            entry.code = code.code;
            if let Some(name) = code.name {
                entry.name = Some(name);
            }
            if let Some(desc) = code.desc {
                entry.description = Some(desc);
            }
            if let Some(group) = code.group {
                entry.group = db.groups.iter().find(|g| g.id == group).map(|g| g.name.clone());
            }
            if let Some(ty) = code.ty {
                entry.ty = match (ty, code.interpretation) {
                    (DatabaseType::Table, None) => ValueType::Table {
                        interpretation: TableInterpretation::Generic,
                    },
                    (DatabaseType::Table, Some(DatabaseInterpretation::Values(..))) =>
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "table type cannot have value names")),
                    (DatabaseType::Table, Some(DatabaseInterpretation::Id(id))) => ValueType::Table {
                        interpretation: match id {
                            DatabaseInterpretationId::CodePage => TableInterpretation::CodePage,
                            id => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("invalid interpretation {:?} for table", id))),
                        },
                    },
                    (DatabaseType::Continuous, ..) => ValueType::Continuous {
                        interpretation: ValueInterpretation::Continuous,
                    },
                    (DatabaseType::NonContinuous, None) => ValueType::NonContinuous {
                        values: Default::default(),
                        interpretation: ValueInterpretation::NonContinuous,
                    },
                    (DatabaseType::NonContinuous, Some(DatabaseInterpretation::Values(values))) => ValueType::NonContinuous {
                        values: values.into_iter()
                            .filter_map(|v| match v.value {
                                DatabaseValue::Value(value) => Some((value, Some(v.name))),
                                DatabaseValue::Range(..) => None, // unimplemented
                            }).collect(),
                        interpretation: ValueInterpretation::NonContinuous,
                    },
                    (DatabaseType::NonContinuous, Some(DatabaseInterpretation::Id(id))) => ValueType::NonContinuous {
                        values: Default::default(),
                        interpretation: match id {
                            DatabaseInterpretationId::NonZeroWrite => ValueInterpretation::NonZeroWrite,
                            DatabaseInterpretationId::VcpVersion => ValueInterpretation::VcpVersion,
                            id => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("invalid interpretation {:?} for nc", id))),
                        },
                    },
                }
            }
            entry.mandatory |= code.mandatory;
            if let Some(access) = code.access {
                entry.access = match access {
                    DatabaseReadWrite::ReadOnly => Access::ReadOnly,
                    DatabaseReadWrite::WriteOnly => Access::WriteOnly,
                    DatabaseReadWrite::ReadWrite => Access::ReadWrite,
                };
            }
            entry.interacts_with.extend(code.interacts_with);
        }

        Ok(())
    }

    fn mccs_database() -> DatabaseFile {
        let data = include_bytes!("../data/mccs.yml");
        serde_yaml::from_slice(data).unwrap()
    }

    /// Create a new database from a specified MCCS specification version.
    pub fn from_version(mccs_version: &Version) -> Self {
        let mut s = Self::default();
        s.apply_database(Self::mccs_database(), mccs_version).unwrap();
        s
    }

    /// Create a new database from a specified database description YAML file.
    ///
    /// This format is not (yet) documented, but an example exists that
    /// [describes the MCCS spec](https://github.com/arcnmx/mccs-rs/blob/master/db/data/mccs.yml).
    pub fn from_database<R: io::Read>(database_yaml: R, mccs_version: &Version) -> io::Result<Self> {
        let db = serde_yaml::from_reader(database_yaml)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut s = Self::default();
        s.apply_database(db, mccs_version)?;
        Ok(s)
    }

    /// Filter out any feature codes or values that are not supported by the
    /// specified display.
    pub fn apply_capabilities(&mut self, caps: &Capabilities) {
        let entries = mem::replace(&mut self.entries, Default::default());
        self.entries.extend(entries.into_iter().filter_map(|(code, desc)| match (caps.vcp_features.get(&code), code, desc) {
            (Some(vcp), code, mut desc) => {
                if let Some(ref name) = vcp.name {
                    desc.name = Some(name.clone());
                }

                if let ValueType::NonContinuous { ref mut values, .. } = desc.ty {
                    let mut full = mem::replace(values, Default::default());
                    values.extend(vcp.values.iter().map(|(&value, caps_name)| match full.remove(&value) {
                        Some(name) => (value, caps_name.clone().or(name)),
                        None => (value, caps_name.clone()),
                    }));
                }

                Some((code, desc))
            },
            _ => None,
        }));
    }

    /// Get the description of a given VCP feature code.
    pub fn get(&self, code: FeatureCode) -> Option<&Descriptor> {
        self.entries.get(&code)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct DatabaseGroup {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DatabaseType {
    Table,
    #[serde(rename = "nc")]
    NonContinuous,
    #[serde(rename = "c")]
    Continuous,
}

#[derive(Debug, Serialize, Deserialize)]
enum DatabaseReadWrite {
    #[serde(rename = "r")]
    ReadOnly,
    #[serde(rename = "w")]
    WriteOnly,
    #[serde(rename = "rw")]
    ReadWrite,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DatabaseValue {
    Value(u8),
    Range(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct DatabaseValueDesc {
    value: DatabaseValue,
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    desc_long: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DatabaseInterpretationId {
    CodePage,
    NonZeroWrite,
    VcpVersion,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DatabaseInterpretation {
    Id(DatabaseInterpretationId),
    Values(Vec<DatabaseValueDesc>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct DatabaseFeature {
    code: FeatureCode,
    #[serde(default)]
    version: version_req::VersionReq,
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(rename="type")]
    ty: Option<DatabaseType>,
    #[serde(default)]
    interpretation: Option<DatabaseInterpretation>,
    #[serde(default)]
    mandatory: bool,
    access: Option<DatabaseReadWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    desc_long: Option<String>,
    #[serde(default, rename = "interacts")]
    interacts_with: Vec<FeatureCode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct DatabaseFile {
    groups: Vec<DatabaseGroup>,
    vcp_features: Vec<DatabaseFeature>,
}

#[test]
fn load_database() {
    for version in &[
        Version::new(2, 0), Version::new(2, 1), Version::new(2, 2),
        Version::new(3, 0)
    ] {
        let db = Database::from_version(version);
        for sample in testdata::test_data() {
            let caps = mccs_caps::parse_capabilities(sample).expect("Failed to parse capabilities");
            let mut db = db.clone();
            db.apply_capabilities(&caps);
            println!("Intersected: {:#?}", db);
        }
    }
}
