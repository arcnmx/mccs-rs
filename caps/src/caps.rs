use {
    super::{bracketed, map_err, trim_spaces, OResult, Value, ValueParser},
    nom::{
        branch::alt,
        bytes::complete::{is_not, tag, take},
        character::complete::{char, space1, u8},
        combinator::{all_consuming, map, map_parser, map_res, opt, rest},
        multi::{fold_many0, many0, separated_list0},
        sequence::{separated_pair, tuple},
        Finish, IResult,
    },
    std::{borrow::Cow, fmt, io, str},
};

#[derive(Clone, PartialEq, Eq)]
pub struct VcpValue {
    pub value: u8,
    pub sub_values: Option<Vec<u8>>,
}

impl VcpValue {
    pub fn new(value: u8) -> Self {
        VcpValue {
            value,
            sub_values: None,
        }
    }

    pub fn sub_values(&self) -> &[u8] {
        self.sub_values.as_ref().map(|v| &v[..]).unwrap_or_default()
    }
}

impl fmt::Debug for VcpValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_tuple("VcpValue");
        let debug = debug.field(&self.value);
        match self.sub_values() {
            &[] => debug.finish(),
            values => debug.field(&values).finish(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Vcp {
    pub feature: u8,
    pub values: Option<Vec<VcpValue>>,
}

impl Vcp {
    pub fn values(&self) -> &[VcpValue] {
        self.values.as_ref().map(|v| &v[..]).unwrap_or_default()
    }
}

impl fmt::Debug for Vcp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_tuple("Vcp");
        let debug = debug.field(&self.feature);
        match self.values() {
            &[] => debug.finish(),
            values => debug.field(&values).finish(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VcpName<'i> {
    pub feature: u8,
    pub name: Option<Cow<'i, str>>,
    pub value_names: Option<Vec<Cow<'i, str>>>,
}

impl<'i> VcpName<'i> {
    pub fn value_names(&self) -> &[Cow<'i, str>] {
        self.value_names.as_ref().map(|v| &v[..]).unwrap_or_default()
    }
}

/// Parsed display capabilities string entry
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cap<'a> {
    Protocol(&'a str),
    Type(&'a str),
    Model(&'a str),
    Commands(Vec<u8>),
    Whql(u8),
    MccsVersion(u8, u8),
    Vcp(Vec<Vcp>),
    VcpNames(Vec<VcpName<'a>>),
    Edid(&'a [u8]),
    Vdif(&'a [u8]),
    Unknown(Value<'a>),
}

impl<'i> Cap<'i> {
    pub fn parse_entries(entries: ValueParser<'i>) -> impl Iterator<Item = io::Result<Cap<'i>>> + 'i {
        entries
            .nom_iter()
            .map(|e| e.and_then(|e| Self::parse_entry(e)).map_err(map_err))
    }

    pub fn parse_entry(value: Value<'i>) -> OResult<'i, Cap<'i>> {
        match value {
            Value::String { tag, value } => Self::parse_string(tag, value),
            Value::Binary { tag, data } => Ok(Self::parse_data(tag, data)),
        }
    }

    pub fn parse_data(tag: &'i str, i: &'i [u8]) -> Cap<'i> {
        match tag {
            "edid" => Cap::Edid(i),
            "vdif" => Cap::Vdif(i),
            _ => Cap::Unknown(Value::Binary { tag, data: i }),
        }
    }

    pub fn parse_string(tag: &'i str, i: &'i [u8]) -> OResult<'i, Cap<'i>> {
        match tag {
            "prot" => all_consuming(map(value, Cap::Protocol))(i),
            "type" => all_consuming(map(value, Cap::Type))(i),
            "model" => all_consuming(map(value, Cap::Model))(i),
            "cmds" => all_consuming(map(hexarray, Cap::Commands))(i),
            "mswhql" => all_consuming(map(map_parser(take(1usize), u8), Cap::Whql))(i),
            "mccs_ver" => all_consuming(map(mccs_ver, |(major, minor)| Cap::MccsVersion(major, minor)))(i),
            // hack for Apple Cinema Display
            "vcp" | "VCP" => all_consuming(map(trim_spaces(many0(vcp)), Cap::Vcp))(i),
            "vcpname" => all_consuming(map(many0(vcpname), Cap::VcpNames))(i),
            _ => Ok((Default::default(), Cap::Unknown(Value::String { tag, value: i }))),
        }
        .finish()
        .map(|(_, c)| c)
    }
}

fn backslash_escape(i: &[u8]) -> IResult<&[u8], String> {
    // TODO: I'd use https://docs.rs/nom/7.1.1/nom/bytes/complete/fn.escaped_transform.html instead,
    // but it can't deal with dynamic transforms due to ExtendInto not being impl'd on anything useful
    // like Vec<u8> or [u8; N] or something...
    let escaped = |i| {
        let (i, _) = tag("\\x")(i)?;
        map_str(take(2usize), |h| u8::from_str_radix(h, 16).map(|v| v as char), i)
    };
    fold_many0(
        alt((
            escaped,
            // TODO: other escapes like \\ \n etc? unclear in access bus spec...
            map(take(1usize), |s: &[u8]| s[0] as char), // TODO, this isn't utf8 parsing, should it be? .-.
        )),
        || String::new(),
        |mut s: String, c| {
            s.push(c);
            s
        },
    )(i)
}

fn value_escape_nospace(i: &[u8]) -> IResult<&[u8], Cow<str>> {
    map_parser(
        is_not(" ()"),
        alt((
            all_consuming(map(map_res(is_not("\\"), str::from_utf8), Cow::Borrowed)),
            map(all_consuming(backslash_escape), Cow::Owned),
        )),
    )(i)
}

fn value(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(rest, str::from_utf8)(i)
}

fn hexarray(i: &[u8]) -> IResult<&[u8], Vec<u8>> {
    many0(trim_spaces(hexvalue))(i)
}

fn map_str<'i, O, E2, F, G>(mut parser: F, f: G, i: &'i [u8]) -> IResult<&'i [u8], O>
where
    F: nom::Parser<&'i [u8], &'i [u8], nom::error::Error<&'i [u8]>>,
    G: FnMut(&'i str) -> Result<O, E2>,
{
    use nom::Parser;

    let mut f = map_res(rest, f);
    let (i, s) = map_res(|i| parser.parse(i), |i| str::from_utf8(i.into()))(i)?;
    match f.parse(s) {
        Ok((_, v)) => Ok((i, v)),
        Err(e) => Err(e.map(|e: nom::error::Error<_>| nom::error::Error { input: i, code: e.code })),
    }
}

fn hexvalue(i: &[u8]) -> IResult<&[u8], u8> {
    map_str(take(2usize), |s| u8::from_str_radix(s, 16), i)
}

fn vcp_value(i: &[u8]) -> IResult<&[u8], VcpValue> {
    map(
        tuple((trim_spaces(hexvalue), opt(bracketed(many0(trim_spaces(hexvalue)))))),
        |(value, sub_values)| VcpValue { value, sub_values },
    )(i)
}

fn vcp(i: &[u8]) -> IResult<&[u8], Vcp> {
    let featurevalues = bracketed(many0(trim_spaces(vcp_value)));
    map(
        tuple((trim_spaces(hexvalue), opt(featurevalues))),
        |(feature, values)| Vcp { feature, values },
    )(i)
}

fn vcpname(i: &[u8]) -> IResult<&[u8], VcpName> {
    let (i, feature) = trim_spaces(hexvalue)(i)?;
    let (i, (name, value_names)) = bracketed(tuple((
        opt(value_escape_nospace),
        opt(bracketed(trim_spaces(separated_list0(space1, value_escape_nospace)))),
    )))(i)?;
    Ok((i, VcpName {
        feature,
        name,
        value_names,
    }))
}

fn mccs_ver(i: &[u8]) -> IResult<&[u8], (u8, u8)> {
    alt((
        separated_pair(u8, char('.'), u8),
        tuple((map_parser(take(2usize), u8), map_parser(take(2usize), u8))),
    ))(i)
}

#[test]
fn vcpname_temp() {
    let testdata = br"14((9300 6500 5500))44(Rotate)80(Do\x20this(On Off))82(Fixit)";
    let expected = [
        VcpName {
            feature: 0x14,
            name: None,
            value_names: Some(vec!["9300".into(), "6500".into(), "5500".into()]),
        },
        VcpName {
            feature: 0x44,
            name: Some("Rotate".into()),
            value_names: None,
        },
        VcpName {
            feature: 0x80,
            name: Some("Do this".into()),
            value_names: Some(vec!["On".into(), "Off".into()]),
        },
        VcpName {
            feature: 0x82,
            name: Some("Fixit".into()),
            value_names: None,
        },
    ];

    let (_, vcps) = all_consuming(many0(vcpname))(testdata).finish().unwrap();
    assert_eq!(vcps.len(), expected.len());

    for (vcp, exp) in vcps.into_iter().zip(expected) {
        assert_eq!(vcp, exp);
    }
}

#[test]
fn vcpname_brightness() {
    let testdata = b"10(Brightness)";
    let expected = VcpName {
        feature: 0x10,
        name: Some("Brightness".into()),
        value_names: None,
    };
    let (_, vcp) = all_consuming(vcpname)(testdata).finish().unwrap();

    assert_eq!(vcp, expected);
}
