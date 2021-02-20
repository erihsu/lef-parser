use crate::{
    model::{
        LefMacro, MacroDensity, MacroOBS, MacroPin, MacroSite, PortGeometry, PortLayerGeometry,
        PortViaGeometry,
    },
    LefRes,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};

use super::base::{float, positive_number, qstring, tstring, ws};
use super::common::{pt, pt_list, rect};
use super::encoder::{
    macro_class_encode, macro_pin_direction_encode, macro_pin_port_class_encode,
    macro_pin_shape_encode, orient_encode, use_type_encode,
};
pub fn macro_parser(input: &str) -> LefRes<&str, LefMacro> {
    context(
        "Macro Statement",
        delimited(
            ws(tag("MACRO")),
            tuple((
                tstring,
                delimited(ws(tag("CLASS")), macro_class_encode, ws(tag(";"))),
                delimited(
                    ws(tag("FOREIGN")),
                    tuple((tstring, opt(pt), opt(orient_encode))),
                    ws(tag(";")),
                ),
                delimited(ws(tag("ORIGIN")), pt, ws(tag(";"))),
                delimited(ws(tag("EEQ")), tstring, ws(tag(";"))),
                delimited(
                    ws(tag("SIZE")),
                    separated_pair(positive_number, ws(tag("BY")), positive_number),
                    ws(tag(";")),
                ),
                delimited(
                    ws(tag("SYMMETRY")),
                    many1(alt((
                        map(tag("X"), |_| 0),
                        map(tag("Y"), |_| 1),
                        map(tag("R90"), |_| 2),
                    ))),
                    ws(tag(";")),
                ),
                many1(site_statement),
                many1(pin_statement),
                obs_statement,
                many1(density_statement),
            )),
            tuple((ws(tag("END")), tstring)),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            LefMacro {
                macro_name: data.0.to_string(),
                macro_class: data.1,
                foreign_cell: ((data.2).0.to_string(), (data.2).1, (data.2).2),
                origin: data.3,
                eeq_macro: data.4.to_string(),
                macro_size: data.5,
                macro_symmetry: data.6,
                macro_site: data.7,
                macro_pin: data.8,
                macro_obs: data.9,
                macro_density: data.10,
            },
        )
    })
}

pub fn site_statement(input: &str) -> LefRes<&str, MacroSite> {
    delimited(
        ws(tag("SITE")),
        tuple((tstring, opt(tstring))),
        ws(tag(";")),
    )(input)
    .map(|(res, data)| {
        (
            res,
            MacroSite {
                site_name: data.0.to_string(),
                site_pattern: data.1.map(|s| s.to_string()),
            },
        )
    })
}

// antenna statement not support
pub fn pin_statement(input: &str) -> LefRes<&str, MacroPin> {
    context(
        "Macro Pin Statement",
        delimited(
            ws(tag("PIN")),
            tuple((
                tstring, // pin name
                delimited(ws(tag("TAPERRULE")), tstring, ws(tag(";"))),
                delimited(
                    ws(tag("DIRECTION")),
                    macro_pin_direction_encode,
                    ws(tag(";")),
                ),
                delimited(ws(tag("USE")), use_type_encode, ws(tag(";"))),
                delimited(ws(tag("NETEXPR")), qstring, ws(tag(";"))),
                delimited(ws(tag("SUPPLYSENSITIVITY")), tstring, ws(tag(";"))),
                delimited(ws(tag("GROUNDSENSITIVITY")), tstring, ws(tag(";"))),
                delimited(ws(tag("SHAPE")), macro_pin_shape_encode, ws(tag(";"))),
                delimited(ws(tag("MUSTJOIN")), tstring, ws(tag(";"))),
                many1(macro_pin_port),
            )),
            tuple((ws(tag("END")), tstring)),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            MacroPin {
                pin_name: data.0.to_string(),
                taper_rule: data.1.to_string(),
                direction: data.2,
                use_type: data.3,
                net_expr: data.4.to_string(),
                supply_sensitivity: data.5.to_string(),
                ground_sensitivity: data.6.to_string(),
                shape: data.7,
                mustjoin: data.8.to_string(),
                macro_port: data.9, // (class,MacroPortObj)
            },
        )
    })
}

// ITERATE syntax not supported
fn port_layer_geometry(input: &str) -> LefRes<&str, PortLayerGeometry> {
    context(
        "Macro Pin Port Statement",
        tuple((
            delimited(
                ws(tag("LAYER")),
                tuple((
                    tstring,
                    map(opt(ws(tag("EXCEPTPGNET"))), |x| match x {
                        None => false,
                        Some(_) => true,
                    }),
                    alt((
                        map(preceded(tag("SPACING"), positive_number), |x| (true, x)),
                        map(preceded(tag("DESIGNRULEWIDTH"), positive_number), |x| {
                            (false, x)
                        }),
                    )),
                )),
                ws(tag(";")),
            ),
            many1(alt((
                map(delimited(ws(tag("PATH")), pt_list, ws(tag(";"))), |x| {
                    PortGeometry::Path(x)
                }),
                map(delimited(ws(tag("RECT")), rect, ws(tag(";"))), |x| {
                    PortGeometry::Rect(x)
                }),
                map(delimited(ws(tag("POLYGON")), pt_list, ws(tag(";"))), |x| {
                    PortGeometry::Polygon(x)
                }),
            ))),
        )),
    )(input)
    .map(|(res, data)| {
        (
            res,
            PortLayerGeometry {
                layer_name: (data.0).0.to_string(),
                if_exceptpgnet: (data.0).1,
                minspacing: (data.0).2, //(if from spacing or designrulewidth, minspacing)
                geometries: data.1,
            },
        )
    })
}

fn port_via_geometry(input: &str) -> LefRes<&str, PortViaGeometry> {
    context(
        "Macro Pin & Obstacle Port Via Statement",
        delimited(ws(tag("VIA")), tuple((pt, tstring)), ws(tag(";"))),
    )(input)
    .map(|(res, data)| {
        (
            res,
            PortViaGeometry {
                via_name: data.1.to_string(),
                via_location: data.0,
            },
        )
    })
}

fn macro_pin_port(input: &str) -> LefRes<&str, (u8, PortLayerGeometry)> {
    context(
        "Macro Pin Port Statement",
        delimited(
            ws(tag("PORT")),
            tuple((
                delimited(ws(tag("CLASS")), macro_pin_port_class_encode, ws(tag(";"))),
                port_layer_geometry,
            )),
            ws(tag("END")),
        ),
    )(input)
}

fn obs_statement(input: &str) -> LefRes<&str, Vec<MacroOBS>> {
    context(
        "Macro Obstacle Statement",
        delimited(
            ws(tag("OBS")),
            many1(alt((
                map(port_layer_geometry, |x| MacroOBS::LayerObj(x)),
                map(port_via_geometry, |x| MacroOBS::ViaObj(x)),
            ))),
            ws(tag("END")),
        ),
    )(input)
}

fn density_statement(input: &str) -> LefRes<&str, MacroDensity> {
    context(
        "
        Macro Density Statement
        ",
        delimited(
            ws(tag("DENSITY")),
            tuple((
                delimited(ws(tag("LAYER")), tstring, ws(tag(";"))),
                many1(delimited(
                    ws(tag("RECT")),
                    tuple((rect, float)),
                    ws(tag(";")),
                )),
            )),
            ws(tag("END")),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            MacroDensity {
                layer_name: data.0.to_string(),
                rect_region: data.1,
            },
        )
    })
}
