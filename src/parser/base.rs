use nom::branch::alt;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of};
use nom::combinator::{map_res, opt, recognize};

use nom::multi::{many0, many1};

use crate::LefRes;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use std::str;
use std::str::FromStr;

// basic parse. Independent from def_parser but it's the most basic parser in def_parser.

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> LefRes<&'a str, O>
where
    F: FnMut(&'a str) -> LefRes<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// // typical string
// // ie. abcdef, de234, jkl_mn, ...
pub fn tstring(input: &str) -> LefRes<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(input)
}

// // parse string that is surrounded by " and ".
// // ie, "abc", "def"
pub fn qstring(input: &str) -> LefRes<&str, &str> {
    ws(recognize(delimited(tag("\""), tstring, tag("\""))))(input)
}

// // unsigned integer number
// // ie, 100, 350
pub fn positive_number(input: &str) -> LefRes<&str, u32> {
    ws(map_res(recognize(digit1), |res: &str| u32::from_str(res)))(input)
}

// parse unsigned floating number
// The following is adapted from the Python parser by Valentin Lorentz (ProgVal).
pub fn float(input: &str) -> LefRes<&str, f32> {
    ws(map_res(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                opt(char('-')),
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((opt(char('-')), decimal, char('.'), opt(decimal)))),
        )),
        |res: &str| f32::from_str(res),
    ))(input)
}

pub fn decimal(input: &str) -> LefRes<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
