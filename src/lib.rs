//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/mccs-rs/")]

#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use std::borrow::Cow;
use std::io;
use nom::*;

pub enum Protocol {
    Monitor,
    Display,
    Unknown(String),
}

pub enum Type {
    CRT,
    LCD,
    LED,
    Unknown(String),
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

pub type Vcp = Vec<(u8, Option<Vec<u8>>)>;
pub type VcpName<'a> = (u8, Option<Cow<'a, str>>, Option<Vec<Cow<'a, str>>>);

#[derive(Debug)]
pub enum Cap<'a> {
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

pub fn parse_capabilities(capability_string: &[u8]) -> io::Result<Vec<Cap>> {
    caps(capability_string).to_result().map_err(|e|
        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
    )
}

#[test]
fn capability_string_samples() {
    let samples = [
        &b"(prot(monitor)type(lcd)27UD58cmds(01 02 03 0C E3 F3)vcp(02 04 05 08 10 12 14(05 08 0B ) 16 18 1A 52 60( 11 12 0F 10) AC AE B2 B6 C0 C6 C8 C9 D6(01 04) DF 62 8D F4 F5(01 02) F6(00 01 02) 4D 4E 4F 15(01 06 11 13 14 28 29 32 48) F7(00 01 02 03) F8(00 01) F9 E4 E5 E6 E7 E8 E9 EA EB EF FD(00 01) FE(00 01 02) FF)mccs_ver(2.1)mswhql(1))"[..],
        &b"(prot(monitor)type(LCD)model(ACER)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 08 0B 10 12 14(05 08 0B) 16 18 1A 52 60(01 03 11) 6C 6E 70 AC AE B2 B6 C6 C8 C9 CC(01 02 03 04 05 06 08 09 0A 0C 0D 14 16 1E) D6(01 05) DF)mswhql(1)asset_eep(40)mccs_ver(2.0))"[..],
        &b"(prot(monitor)type(LED)model(25UM65)cmds(01 02 03 0C E3 F3)vcp(0203(10 00)0405080B0C101214(05 07 08 0B) 16181A5260(03 04)6C6E7087ACAEB6C0C6C8C9D6(01 04)DFE4E5E6E7E8E9EAEBED(00 10 20 40)EE(00 01)FE(01 02 03)FF)mswhql(1)mccs_ver(2.1))"[..], // example from ddcutil
        &b"Prot(display) type(lcd) model(xxxxx) cmds(xxxxx) vcp(02 03 10 12 C8 DC(00 01 02 03 07) DF)mccs_ver(2.2) window1(type (PIP) area(25 25 1895 1175) max(640 480) min(10 10) window(10))vcpname(10(Brightness))"[..], // example from MCCS spec v2.2a
        &br"vcpname(14((9300 6500 5500))44(Rotate)80(Do\x20this(On Off))82(Fixit))"[..], // example from access bus section 7
        // tagged length with bracket and invalid utf8 seems like a worst case scenario here:
        &b"edid bin(3(\xff) ))vdif bin(3 (abc))unknown bin(2(ab))"[..],
    ];

    for sample in &samples {
        let caps = caps(sample).to_full_result().expect("Failed to parse capabilities");
        println!("Caps: {:?}", caps);
    }
}
