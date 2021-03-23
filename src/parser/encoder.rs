// common scope
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::sequence::tuple;

use crate::LefRes;

use super::base::ws;
// use super::def_types::{
//     DirecttionT, LocAtrriT, OrientT, PatternT, RegionT, ShapeT, SourceT, UseModeT,
// };

pub fn orient_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        map(tag("N"), |_| 0),
        map(tag("W"), |_| 1),
        map(tag("S"), |_| 2),
        map(tag("E"), |_| 3),
        map(tag("FN"), |_| 4),
        map(tag("FW"), |_| 5),
        map(tag("FS"), |_| 6),
        map(tag("FE"), |_| 7),
    )))(input)
}

pub fn use_type_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        map(tag("SIGNAL"), |_| 0),
        map(tag("ANALOG"), |_| 1),
        map(tag("POWER"), |_| 2),
        map(tag("GROUND"), |_| 3),
        map(tag("CLOCK"), |_| 4),
    )))(input)
}

pub fn macro_class_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        alt((
            map(tuple((tag("COVER"), space1, tag("BUMP"))), |_| 0),
            map(tag("COVER"), |_| 1),
        )),
        map(tag("RING"), |_| 2),
        alt((
            map(tuple((tag("BLOCK"), space1, tag("BLACKBOX"))), |_| 3),
            map(tuple((tag("BLOCK"), space1, tag("SOFT"))), |_| 4),
            map(tag("BLOCK"), |_| 5),
        )),
        alt((
            map(tuple((tag("PAD"), space1, tag("INPUT"))), |_| 6),
            map(tuple((tag("PAD"), space1, tag("OUTPUT"))), |_| 7),
            map(tuple((tag("PAD"), space1, tag("INOUT"))), |_| 8),
            map(tuple((tag("PAD"), space1, tag("POWER"))), |_| 9),
            map(tuple((tag("PAD"), space1, tag("SPACER"))), |_| 10),
            map(tuple((tag("PAD"), space1, tag("AREAIO"))), |_| 11),
            map(tag("PAD"), |_| 12),
        )),
        alt((
            map(tuple((tag("CORE"), space1, tag("FEEDTRHU"))), |_| 13),
            map(tuple((tag("CORE"), space1, tag("TIEHIGH"))), |_| 14),
            map(tuple((tag("CORE"), space1, tag("TIELOW"))), |_| 15),
            map(tuple((tag("CORE"), space1, tag("SPACER"))), |_| 16),
            map(tuple((tag("CORE"), space1, tag("ANTENNACELL"))), |_| 17),
            map(tuple((tag("CORE"), space1, tag("WELLTAP"))), |_| 18),
            map(tag("CORE"), |_| 19),
        )),
        alt((
            map(tuple((tag("ENDCAP"), space1, tag("PRE"))), |_| 20),
            map(tuple((tag("ENDCAP"), space1, tag("POST"))), |_| 21),
            map(tuple((tag("ENDCAP"), space1, tag("TOPLEFT"))), |_| 22),
            map(tuple((tag("ENDCAP"), space1, tag("TOPRIGHT"))), |_| 23),
            map(tuple((tag("ENDCAP"), space1, tag("BOTTOMLEFT"))), |_| 24),
            map(tuple((tag("ENDCAP"), space1, tag("BOTTOMRIGHT"))), |_| 25),
            map(tag("ENDCAP"), |_| 26),
        )),
    )))(input)
}

pub fn macro_pin_direction_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        map(tag("INPUT"), |_| 0),
        map(tuple((tag("OUTPUT"), space1, tag("TRISTATE"))), |_| 1),
        map(tag("OUTPUT"), |_| 2),
        map(tag("INOUT"), |_| 3),
        map(tag("FEEDTHRU"), |_| 4),
    )))(input)
}

pub fn macro_pin_shape_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        map(tag("ABUTMENT"), |_| 0),
        map(tag("RING"), |_| 1),
        map(tag("FEEDTHRU"), |_| 2),
    )))(input)
}

pub fn macro_pin_port_class_encode(input: &str) -> LefRes<&str, u8> {
    ws(alt((
        map(tag("NONE"), |_| 0),
        map(tag("CORE"), |_| 1),
        map(tag("BUMP"), |_| 2),
    )))(input)
}

// pub fn antenna_model_encode(input: &str) -> LefRes<&str, u8> {
//     ws(alt((
//         map(tag("OXIDE1"), |_| 0),
//         map(tag("OXIDE2"), |_| 1),
//         map(tag("OXIDE3"), |_| 2),
//         map(tag("OXIDE4"), |_| 3),
//     )))(input)
// }
