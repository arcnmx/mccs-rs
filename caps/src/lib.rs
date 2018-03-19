#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/mccs-rs/")]

//! MCCS compliant displays will report their supported capabilities in a string
//! retrieved over DDC/CI. The format of this string is specified in the DDC
//! specification, MCCS, and ACCESS.bus section 7. This crate parses the
//! capability string into structured data.

#[macro_use]
extern crate nom;
extern crate mccs;

use std::str::{self, FromStr};
use std::borrow::Cow;
use std::io;
use mccs::{Capabilities, Version, VcpDescriptor, UnknownTag, UnknownData};
use nom::{digit, is_space, is_alphanumeric};

#[cfg(test)]
mod testdata;

/// Parses a MCCS capability string.
pub fn parse_capabilities<C: AsRef<[u8]>>(capability_string: C) -> io::Result<Capabilities> {
    caps(capability_string.as_ref()).to_result().map_err(|e|
        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
    ).map(|c| {
        // TODO: check for multiple tags of anything only allowed once?

        let mut caps = Capabilities::default();
        for cap in &c {
            match *cap {
                Cap::Protocol(protocol) => caps.protocol = Some(protocol.into()),
                Cap::Type(ty) => caps.ty = Some(ty.into()),
                Cap::Model(model) => caps.model = Some(model.into()),
                Cap::Commands(ref cmds) => caps.commands = cmds.clone(),
                Cap::Whql(whql) => caps.ms_whql = Some(whql),
                Cap::MccsVersion(major, minor) => caps.mccs_version = Some(Version::new(major, minor)),
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

type Vcp = Vec<(u8, Option<Vec<u8>>)>;
type VcpName<'a> = (u8, Option<Cow<'a, str>>, Option<Vec<Cow<'a, str>>>);

#[derive(Debug)]
enum Cap<'a> {
    Protocol(&'a str),
    Type(&'a str),
    Model(&'a str),
    Commands(Vec<u8>),
    Whql(u8),
    MccsVersion(u8, u8),
    Vcp(Vcp),
    VcpNames(Vec<VcpName<'a>>),
    Unknown(&'a str, &'a str),
    UnknownBytes(&'a str, &'a [u8]),
    UnknownBinary(&'a str, &'a [u8]),
    Edid(&'a [u8]),
    Vdif(&'a [u8]),
}

named!(hexarray<&[u8], Vec<u8>>,
    many0!(hexvalue)
);

named!(hexvalue<&[u8], u8>,
    do_parse!(
        take_while!(is_space) >>
        v: map_res!(complete!(take_str!(2)), |h| u8::from_str_radix(h, 16)) >>
        take_while!(is_space) >>
        (v)
    )
);

named!(vcp<&[u8], Vcp>,
    delimited!(
        char!('('),
        many0!(
            do_parse!(
                v: hexvalue >>
                c: opt!(delimited!(
                    char!('('),
                    hexarray,
                    char!(')')
                )) >>
                (v, c)
            )
        ),
        char!(')')
    )
);

named!(balancedparens,
    take_while!({
        // I have no idea how to thread state through this so yay globals...
        use std::sync::atomic::{Ordering, AtomicUsize};
        static DEPTH: AtomicUsize = AtomicUsize::new(0);
        move |c| {
            let depth = DEPTH.load(Ordering::Relaxed);

            match c {
                b')' if depth == 0 => false,
                b')' => {
                    DEPTH.store(depth - 1, Ordering::Relaxed);
                    true
                },
                b'(' => {
                    DEPTH.store(depth + 1, Ordering::Relaxed);
                    true
                },
                _ => true,
            }
        }
    })
);

named!(ident<&[u8], &str>,
    map_res!(take_while!(|c| is_alphanumeric(c) || c == b'_'), str::from_utf8)
);

named!(backslash_escape<&[u8], String>,
    fold_many0!(
        alt!(
            do_parse!(
                tag!("\\x") >>
                v: map_res!(complete!(take_str!(2)), |h| u8::from_str_radix(h, 16).map(|v| v as char)) >>
                (v)
            ) |
            // TODO: other escapes like \\ \n etc? unclear in access bus spec...
            map!(complete!(take_str!(1)), |s| s.chars().next().unwrap())
        ),
        String::new(),
        |mut s: String, c| {
            s.push(c);
            s
        }
    )
);

named!(value_escape_nospace<&[u8], Cow<str>>,
    flat_map!(
        is_not!(" ()"),
        alt!(
            do_parse!(
                v: map!(map_res!(is_not!("\\"), str::from_utf8), Cow::Borrowed) >>
                eof!() >>
                (v)
            ) |
            map!(
                do_parse!(
                    v: backslash_escape >>
                    eof!() >>
                    (v)
                ),
                Cow::Owned
            )
        )
    )
    //map!(map_res!(is_not!("\\"), str::from_utf8), Cow::Borrowed)
);

named!(value<&[u8], &str>,
    map_res!(is_not!("()"), str::from_utf8)
);

named!(caps<&[u8], Vec<Cap>>,
    do_parse!(
        v: alt!(
            delimited!(
                char!('('),
                caps_inner,
                char!(')')
            ) |
            caps_inner // hack for Apple Cinema Display
        ) >>
        eof!() >>
        (v)
    )
);

named!(binary,
    do_parse!(
        tag!(" bin") >>
        take_while!(is_space) >>
        v: delimited!(
            char!('('),
            do_parse!(
                count: map_res!(map_res!(digit, str::from_utf8), usize::from_str) >>
                take_while!(is_space) >>
                data: delimited!(
                    char!('('),
                    complete!(take!(count)),
                    char!(')')
                ) >>
                (data)
            ),
            char!(')')
        ) >>
        (v)
    )
);

// TODO: use tag_no_case?

named!(caps_inner<&[u8], Vec<Cap>>,
    many0!(
        do_parse!(
            take_while!(is_space) >>
            v: alt!(
                do_parse!(
                    tag!("prot") >>
                    v: delimited!(
                        char!('('),
                        value,
                        char!(')')
                    ) >>
                    (Cap::Protocol(v))
                ) |
                do_parse!(
                    tag!("type") >>
                    v: delimited!(
                        char!('('),
                        value,
                        char!(')')
                    ) >>
                    (Cap::Type(v))
                ) |
                do_parse!(
                    tag!("model") >>
                    v: delimited!(
                        char!('('),
                        value,
                        char!(')')
                    ) >>
                    (Cap::Model(v))
                ) |
                do_parse!(
                    tag!("cmds") >>
                    v: delimited!(
                        char!('('),
                        hexarray,
                        char!(')')
                    ) >>
                    (Cap::Commands(v))
                ) |
                do_parse!(
                    tag!("mswhql") >>
                    v: delimited!(
                        char!('('),
                        map_res!(take_str!(1), u8::from_str),
                        char!(')')
                    ) >>
                    (Cap::Whql(v))
                ) |
                do_parse!(
                    tag!("mccs_ver") >>
                    v: delimited!(
                        char!('('),
                        do_parse!(
                            major: map_res!(digit, |v| u8::from_str(str::from_utf8(v).unwrap())) >>
                            tag!(".") >>
                            minor: map_res!(digit, |v| u8::from_str(str::from_utf8(v).unwrap())) >>
                            (major, minor)
                        ),
                        char!(')')
                    ) >>
                    (Cap::MccsVersion(v.0, v.1))
                ) |
                do_parse!(
                    alt!(tag!("vcp") | tag!("VCP")) >> // hack for Apple Cinema Display
                    v: vcp >>
                    (Cap::Vcp(v))
                ) |
                do_parse!(
                    tag!("vcpname") >>
                    v: delimited!(
                        char!('('),
                        many0!(
                            do_parse!(
                                f: hexvalue >>
                                v: delimited!(
                                    char!('('),
                                    do_parse!(
                                        v: opt!(value_escape_nospace) >>
                                        n: opt!(delimited!(
                                            char!('('),
                                            many0!(
                                                do_parse!(
                                                    take_while!(is_space) >>
                                                    value: value_escape_nospace >>
                                                    take_while!(is_space) >>
                                                    (value)
                                                )
                                            ),
                                            char!(')')
                                        )) >>
                                        (v, n)
                                    ),
                                    char!(')')
                                ) >>
                                (f, v.0, v.1)
                            )
                        ),
                        char!(')')
                    ) >>
                    (Cap::VcpNames(v))
                ) |
                do_parse!(
                    tag!("edid") >>
                    v: binary >>
                    (Cap::Edid(v))
                ) |
                do_parse!(
                    tag!("vdif") >>
                    v: binary >>
                    (Cap::Vdif(v))
                ) |
                do_parse!(
                    tag: ident >>
                    v: binary >>
                    (Cap::UnknownBinary(tag, v))
                ) |
                do_parse!(
                    tag: ident >>
                    v: delimited!(
                        char!('('),
                        map_res!(balancedparens, str::from_utf8),
                        char!(')')
                    ) >>
                    (Cap::Unknown(tag, v))
                ) |
                do_parse!(
                    tag: ident >>
                    v: delimited!(
                        char!('('),
                        balancedparens,
                        char!(')')
                    ) >>
                    (Cap::UnknownBytes(tag, v))
                )
            ) >>
            take_while!(is_space) >>
            (v)
        )
    )
);

#[test]
fn samples_raw() {
    for sample in testdata::test_data() {
        println!("Parsing caps: {}", String::from_utf8_lossy(sample));
        let caps = caps(sample).to_full_result().expect("Failed to parse capabilities");
        println!("Caps: {:?}", caps);
    }
}

#[test]
fn samples_high() {
    for sample in testdata::test_data() {
        let caps = parse_capabilities(sample).expect("Failed to parse capabilities");
        println!("Caps: {:#?}", caps);
    }
}
