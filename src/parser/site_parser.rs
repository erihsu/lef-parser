// use super::encoder::{net_pattern_encode, source_type_encode, use_mode_encode};
use crate::{model::LefSite, LefRes};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, tuple};

use super::base::{positive_number, tstring, ws};

use super::encoder::orient_encode;
fn site_rowpattern(input: &str) -> LefRes<&str, (&str, u8)> {
    context(
        "Site Row Pattern Statement",
        tuple((tstring, orient_encode)),
    )(input)
}

fn site_symmetry(input: &str) -> LefRes<&str, u8> {
    context(
        "Site Symmetry Statement",
        alt((
            map(tag("X"), |_| 0),
            map(tag("Y"), |_| 1),
            map(tag("R90"), |_| 2),
        )),
    )(input)
}

pub fn site_parser(input: &str) -> LefRes<&str, LefSite> {
    context(
        "Site Statement",
        delimited(
            ws(tag("SITE")),
            tuple((
                tstring,
                delimited(
                    ws(tag("CLASS")),
                    alt((map(tag("PAD"), |_| true), map(tag("CORE"), |_| false))),
                    ws(tag(";")),
                ),
                opt(delimited(
                    ws(tag("CLASS")),
                    many1(site_symmetry),
                    ws(tag(";")),
                )),
                opt(delimited(
                    ws(tag("ROWPATTERN")),
                    many1(site_rowpattern),
                    ws(tag(";")),
                )),
                tuple((
                    preceded(ws(tag("SIZE")), positive_number),
                    preceded(ws(tag("BY")), positive_number),
                )),
            )),
            tuple((ws(tag("SITE")), tstring)),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            LefSite {
                site_name: data.0.to_string(),
                site_class: data.1,
                site_symmetry: data.2,
                row_pattern: data
                    .3
                    .map(|x| x.iter().map(|y| (y.0.to_string(), y.1)).collect()),
                site_size: data.4,
            },
        )
    })
}
