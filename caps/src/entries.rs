use {
    super::{bracketed, map_err, trim_spaces, OResult, OResultI, Value},
    nom::{
        branch::alt,
        bytes::complete::{tag, take, take_while1, take_while_m_n},
        character::{
            complete::{alphanumeric1, char, space0, u32},
            is_alphanumeric,
        },
        combinator::{complete, fail, map, map_res, not, peek},
        error::{self, ErrorKind},
        sequence::{preceded, tuple},
        IResult, Parser,
    },
    std::{io, str},
};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ValueParser<'i> {
    pub input: &'i [u8],
    pub brackets: Option<usize>,
    previous_tag: Option<&'i str>,
}

impl<'i> Iterator for ValueParser<'i> {
    type Item = io::Result<Value<'i>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None
        }

        Some(self.nom_result().map_err(map_err))
    }
}

impl<'i> ValueParser<'i> {
    pub fn new(capability_string: &'i [u8]) -> ValueParser<'i> {
        Self {
            input: capability_string,
            brackets: None,
            previous_tag: None,
        }
    }

    pub fn nom_iter(mut self) -> impl Iterator<Item = OResult<'i, Value<'i>>> + 'i {
        std::iter::from_fn(move || match self.input.is_empty() {
            true => None,
            false => Some(self.nom_result()),
        })
    }

    pub fn nom_result(&mut self) -> OResult<'i, Value<'i>> {
        match self.nom() {
            Ok(o) => Ok(o),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
            Err(nom::Err::Incomplete(_)) => Err(error::Error::new(self.input, ErrorKind::Eof)),
        }
    }

    pub fn nom(&mut self) -> OResultI<'i, Value<'i>> {
        self.parse(self.input).map(|(_, e)| e)
    }
}

impl<'i> Parser<&'i [u8], Value<'i>, error::Error<&'i [u8]>> for ValueParser<'i> {
    fn parse(&mut self, input: &'i [u8]) -> IResult<&'i [u8], Value<'i>> {
        let (input, mut brackets) = match self.brackets {
            None => {
                let (input, brackets) = caps_prefix(input)?;
                self.input = input;
                self.brackets = Some(brackets);
                (input, brackets)
            },
            Some(brackets) => (input, brackets),
        };

        let (input, e) = Value::parse_nom(input, self.previous_tag)?;
        self.previous_tag = Some(e.tag());
        self.input = input;

        let input = match caps_suffix(brackets, input) {
            Ok((rest, _)) if rest == input => input,
            Ok((input, brackets_consumed)) => {
                brackets -= brackets_consumed;
                self.input = input;
                self.brackets = Some(brackets);
                input
            },
            Err(_) => unreachable!(),
        };

        Ok((input, e))
    }
}

fn caps_prefix(i: &[u8]) -> IResult<&[u8], usize> {
    // hack around Apple Cinema Display and other displays without any surrounding brackets
    // and displays with too many brackets
    let (i, brackets) = take_while_m_n(0, 2, |c| c == b'(')(i)?;
    Ok((i, brackets.len()))
}

fn caps_suffix(mut brackets: usize, mut i: &[u8]) -> IResult<&[u8], usize> {
    let mut bracket_count = 0;
    loop {
        i = match i.split_first() {
            Some((&b')', i)) => match brackets.checked_sub(1) {
                Some(b) => {
                    brackets = b;
                    bracket_count += 1;
                    i
                },
                None => break,
            },
            Some((&0 | &b' ', i)) => i,
            _ => break,
        };
    }
    Ok((i, bracket_count))
}

impl<'i> Value<'i> {
    pub fn parse_nom(input: &'i [u8], previous_tag: Option<&'i str>) -> IResult<&'i [u8], Self> {
        let (i, _) = space0(input)?;
        let (i, id) = alt((
            map(
                preceded(tuple((tag("model"), peek(not(char('('))))), modelhack),
                |value| Err(Value::String { tag: "model", value }),
            ),
            map(
                |i| {
                    if previous_tag == Some("type") {
                        modelhack(i)
                    } else {
                        fail(i)
                    }
                },
                |value| Err(Value::String { tag: "model", value }),
            ),
            map(ident, Ok),
        ))(i)?;
        let (i, cap) = match id {
            Ok(id) => alt((
                map(preceded(tag(" bin"), bracketed(binary)), |data| Value::Binary {
                    tag: id,
                    data,
                }),
                map(bracketed(balancedparens), |value| Value::String { tag: id, value }),
            ))(i),
            Err(corrupted_model) => Ok((i, corrupted_model)),
        }?;
        let (i, _) = space0(i)?;
        Ok((i, cap))
    }

    pub fn nom_parser() -> ValueParser<'i> {
        Default::default()
    }
}

fn binary(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, count) = trim_spaces(u32)(i)?;
    bracketed(take(count))(i)
}

fn modelhack(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let cmds = b"cmds";
    let (rest, model) = alphanumeric1(i)?;
    if !model.ends_with(cmds) || model == cmds {
        let (_, ()) = fail(i)?;
    }
    let _ = peek(char('('))(rest)?;
    let model = &model[..model.len() - 4];
    Ok((&i[model.len()..], model))
}

fn ident(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while1(|c| is_alphanumeric(c) || c == b'_'), str::from_utf8)(i)
}

fn balancedparens(i: &[u8]) -> IResult<&[u8], &[u8]> {
    complete(balancedparens_incomplete)(i)
}

fn balancedparens_incomplete(i: &[u8]) -> IResult<&[u8], &[u8]> {
    use nom::InputTake;

    let mut depth = 0usize;
    for (x, &c) in i.iter().enumerate() {
        match c {
            b')' => match depth.checked_sub(1) {
                Some(v) => depth = v,
                None => return Ok(i.take_split(x)),
            },
            b'(' => {
                depth += 1;
            },
            _ => (),
        }
    }
    match depth {
        0 => Ok((i, Default::default())),
        depth => Err(nom::Err::Incomplete(nom::Needed::new(depth))),
    }
}

#[test]
fn model_hacks() {
    let testdata = [
        (
            &[
                &b"type(lcd)ABCdcmds()"[..],
                &b"type(lcd)modelABCdcmds()"[..],
                &b"type(lcd)model(ABCd)cmds()"[..],
            ][..],
            &[
                Value::String {
                    tag: "type",
                    value: b"lcd",
                },
                Value::String {
                    tag: "model",
                    value: b"ABCd",
                },
                Value::String {
                    tag: "cmds",
                    value: b"",
                },
            ][..],
        ),
        (
            &[&b"type(lcd)cmds()model(ABCd)"[..]][..],
            &[
                Value::String {
                    tag: "type",
                    value: b"lcd",
                },
                Value::String {
                    tag: "cmds",
                    value: b"",
                },
                Value::String {
                    tag: "model",
                    value: b"ABCd",
                },
            ][..],
        ),
    ];

    for (testdatas, expected) in testdata {
        for testdata in testdatas {
            let testdata: Result<Vec<_>, _> = ValueParser::new(testdata).collect();
            assert_eq!(testdata.unwrap(), expected);
        }
    }
}

#[test]
fn bin_entries() {
    let testdata = b"edid bin(3(\xff) ))vdif bin(3 (abc))unknown bin(2(ab))";
    let expected = [
        Value::Binary {
            tag: "edid",
            data: b"\xff) ",
        },
        Value::Binary {
            tag: "vdif",
            data: b"abc",
        },
        Value::Binary {
            tag: "unknown",
            data: b"ab",
        },
    ];

    let entries: Result<Vec<_>, _> = ValueParser::new(testdata).collect();
    assert_eq!(entries.unwrap(), expected);
}
