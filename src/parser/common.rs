use super::base::{number, number_str, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
// use super::def_types::{Geometry, NetCommonProperty, PropValue, Properties, RtPt};

use nom::character::complete::{char, not_line_ending, space0};
use nom::combinator::{map, value};

use crate::LefRes;

// use super::encoder::{net_pattern_encode, source_type_encode, use_mode_encode};
// common parser used in def_parser. These parser are very commonly used in def_parser so collect them together.

pub fn pt_str(input: &str) -> LefRes<&str, (&str, &str)> {
    delimited(
        ws(tag("(")),
        separated_pair(
            alt((tag("*"), number_str)),
            space0,
            alt((tag("*"), number_str)),
        ),
        ws(tag(")")),
    )(input)
}

pub fn pt(input: &str) -> LefRes<&str, (i32, i32)> {
    delimited(
        ws(tag("(")),
        separated_pair(number, space0, number),
        ws(tag(")")),
    )(input)
}

pub fn rect(input: &str) -> LefRes<&str, ((i32, i32), (i32, i32))> {
    tuple((
        delimited(
            ws(tag("(")),
            separated_pair(number, space0, number),
            ws(tag(")")),
        ),
        delimited(
            ws(tag("(")),
            separated_pair(number, space0, number),
            ws(tag(")")),
        ),
    ))(input)
}

pub fn pt_list(input: &str) -> LefRes<&str, Vec<(i32, i32)>> {
    map(many1(pt_str), |res: Vec<(&str, &str)>| {
        let mut out = Vec::new();
        let mut prev_x = 0;
        let mut prev_y = 0;
        for (pt_x, pt_y) in res {
            prev_x = match pt_x.parse::<i32>() {
                Ok(n) => n,
                Err(_) => prev_x,
            };
            prev_y = match pt_y.parse::<i32>() {
                Ok(n) => n,
                Err(_) => prev_y,
            };
            let a_pt = (prev_x, prev_y);
            out.push(a_pt);
        }
        out
    })(input)
}

// modified from nom::recipies
pub fn lef_comment(input: &str) -> LefRes<&str, ()> {
    value(
        (), // Output is thrown away.
        preceded(ws(char('#')), not_line_ending),
    )(input)
}
