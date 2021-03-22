use super::base::ws;
use super::header_parser::*;
use super::macro_parser::*;
use super::site_parser::*;
use crate::{model::LefData, LefRes};
use nom::bytes::complete::tag;

use nom::error::context;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::sequence::tuple;
pub fn lef_parser(input: &str) -> LefRes<&str, LefData> {
    context(
        "Cell LEF",
        terminated(
            tuple((header_section, site_parser, many1(macro_parser))),
            tuple((ws(tag("END")), ws(tag("LIBRARY")))),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            LefData {
                version: (data.0).1.map_or(5.7, |s| s),
                dividechar: (data.0).2.map_or("/".to_string(), |s| s.to_string()),
                busbitchar: (data.0).3.map_or("[]".to_string(), |s| s.to_string()),
                site: data.1,
                macro_: data.2,
            },
        )
    })
}
