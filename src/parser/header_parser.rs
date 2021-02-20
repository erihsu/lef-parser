use crate::LefRes;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

use super::base::{float, ws};
use super::common::lef_comment;

pub fn header_section(
    input: &str,
) -> LefRes<&str, (Vec<()>, Option<f32>, Option<&str>, Option<&str>)> {
    tuple((
        many0(lef_comment),
        opt(version_num),
        opt(divider_char),
        opt(busbit_chars),
    ))(input)
}

fn divider_char(input: &str) -> LefRes<&str, &str> {
    delimited(
        ws(tag("DIVIDERCHAR")),
        alt((
            ws(tag("\"/\"")),
            ws(tag("\"\\\"")),
            ws(tag("\"%\"")),
            ws(tag("\"$\"")),
        )),
        ws(tag(";")),
    )(input)
}
fn busbit_chars(input: &str) -> LefRes<&str, &str> {
    delimited(
        ws(tag("BUSBITCHARS")),
        alt((ws(tag("\"[]\"")), ws(tag("\"{}\"")), ws(tag("\"<>\"")))),
        ws(tag(";")),
    )(input)
}

// parse version number
fn version_num(
    input: &str,
) -> LefRes<
    &str,
    f32, // version number
> {
    delimited(ws(tag("VERSION")), float, ws(tag(";")))(input)
}
