use super::base::float;

use nom::bytes::complete::is_not;
use nom::multi::many1;
use nom::sequence::{pair, separated_pair, tuple};
// use super::def_types::{Geometry, NetCommonProperty, PropValue, Properties, RtPt};

use nom::character::complete::{char, space0};
use nom::combinator::value;

use crate::LefRes;

// use super::encoder::{net_pattern_encode, source_type_encode, use_mode_encode};
// common parser used in def_parser. These parser are very commonly used in def_parser so collect them together.

pub fn pt(input: &str) -> LefRes<&str, (f32, f32)> {
    separated_pair(float, space0, float)(input)
}

pub fn rect(input: &str) -> LefRes<&str, ((f32, f32), (f32, f32))> {
    tuple((tuple((float, float)), tuple((float, float))))(input)
}

pub fn pt_list(input: &str) -> LefRes<&str, Vec<(f32, f32)>> {
    many1(pt)(input)
}

// modified from nom::recipies
pub fn lef_comment(input: &str) -> LefRes<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(char('#'), is_not("\n")),
    )(input)
}
