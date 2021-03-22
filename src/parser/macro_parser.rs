use crate::{
    model::{
        LefMacro, MacroDensity, MacroPin, MacroPinAntenna, MacroSite, PortGeometry, PortShape,
    },
    LefRes,
};
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};

use super::base::{float, positive_number, qstring, tstring, ws};
use super::common::{pt, pt_list, rect};
use super::encoder::{
    antenna_model_encode, macro_class_encode, macro_pin_direction_encode,
    macro_pin_port_class_encode, macro_pin_shape_encode, orient_encode, use_type_encode,
};
pub fn macro_parser(input: &str) -> LefRes<&str, LefMacro> {
    context(
        "Macro Statement",
        delimited(
            ws(tag("MACRO")),
            permutation((
                tstring,
                delimited(ws(tag("CLASS")), macro_class_encode, ws(tag(";"))),
                delimited(
                    ws(tag("FOREIGN")),
                    tuple((tstring, opt(pt), opt(orient_encode))),
                    ws(tag(";")),
                ),
                delimited(ws(tag("ORIGIN")), pt, ws(tag(";"))),
                delimited(
                    ws(tag("SIZE")),
                    separated_pair(float, ws(tag("BY")), float),
                    ws(tag(";")),
                ),
                delimited(
                    ws(tag("SYMMETRY")),
                    many1(alt((
                        map(ws(tag("X")), |_| 0),
                        map(ws(tag("Y")), |_| 1),
                        map(ws(tag("R90")), |_| 2),
                    ))),
                    ws(tag(";")),
                ),
                many1(site_statement),
                many1(pin_statement),
                obs_statement,
                opt(density_statement),
                opt(delimited(ws(tag("EEQ")), tstring, ws(tag(";")))),
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
                eeq_macro: data.10.map(|s| s.to_string()),
                macro_size: data.4,
                macro_symmetry: data.5,
                macro_site: data.6,
                macro_pin: data.7,
                macro_obs: data.8,
                macro_density: data.9,
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
                // basic infomation, must declare
                tstring, // pin name
                // 1
                opt(delimited(ws(tag("TAPERRULE")), tstring, ws(tag(";")))),
                // 2
                delimited(
                    ws(tag("DIRECTION")),
                    macro_pin_direction_encode,
                    ws(tag(";")),
                ),
                // 3
                opt(delimited(ws(tag("USE")), use_type_encode, ws(tag(";")))),
                // 4
                opt(delimited(ws(tag("NETEXPR")), qstring, ws(tag(";")))),
                // 5
                opt(delimited(
                    ws(tag("SUPPLYSENSITIVITY")),
                    tstring,
                    ws(tag(";")),
                )),
                // 6
                opt(delimited(
                    ws(tag("GROUNDSENSITIVITY")),
                    tstring,
                    ws(tag(";")),
                )),
                // 7
                opt(delimited(
                    ws(tag("SHAPE")),
                    macro_pin_shape_encode,
                    ws(tag(";")),
                )),
                // 8
                opt(delimited(ws(tag("MUSTJOIN")), tstring, ws(tag(";")))),
                // 9
                macro_pin_port,
                // 10
                opt(pin_antenna_statement),
            )),
            tuple((ws(tag("END")), tstring)),
        ),
    )(input)
    .map(|(res, data)| {
        (
            res,
            MacroPin {
                pin_name: data.0.to_string(),
                direction: data.2,
                pin_port: (data.9).1, // (class,MacroPortObj)
                use_type: data.3.map_or(0, |s| s),
                shape: data.7,
                taper_rule: data.1.map(|x| x.to_string()),
                net_expr: data.4.map(|x| x.to_string()),
                supply_sensitivity: data.5.map(|x| x.to_string()),
                ground_sensitivity: data.6.map(|x| x.to_string()),
                mustjoin: data.8.map(|x| x.to_string()),
                pin_antenna: data.10,
            },
        )
    })
}

// ITERATE syntax not supported
fn port_geometry(input: &str) -> LefRes<&str, PortShape> {
    context(
        "Macro Pin Layer Geometry Statement",
        tuple((
            delimited(
                ws(tag("LAYER")),
                tuple((
                    tstring,
                    map(opt(ws(tag("EXCEPTPGNET"))), |x| match x {
                        None => false,
                        Some(_) => true,
                    }),
                    opt(alt((
                        map(preceded(tag("SPACING"), positive_number), |x| (true, x)),
                        map(preceded(tag("DESIGNRULEWIDTH"), positive_number), |x| {
                            (false, x)
                        }),
                    ))),
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
                map(
                    delimited(ws(tag("VIA")), tuple((pt, tstring)), ws(tag(";"))),
                    |x| PortGeometry::Via((x.1.to_string(), x.0)),
                ),
            ))),
        )),
    )(input)
    .map(|(res, data)| {
        (
            res,
            PortShape {
                layer_name: (data.0).0.to_string(),
                geometries: data.1,
            },
        )
    })
}

// fn port_via_geometry(input: &str) -> LefRes<&str, PortViaGeometry> {
//     context(
//         "Macro Pin & Obstacle Port Via Statement",
//         delimited(ws(tag("VIA")), tuple((pt, tstring)), ws(tag(";"))),
//     )(input)
//     .map(|(res, data)| {
//         (
//             res,
//             PortViaGeometry {
//                 via_name: data.1.to_string(),
//                 via_location: data.0,
//             },
//         )
//     })
// }

fn macro_pin_port(input: &str) -> LefRes<&str, (Option<u8>, Vec<PortShape>)> {
    context(
        "Macro Pin Port Statement",
        delimited(
            ws(tag("PORT")),
            tuple((
                opt(delimited(
                    ws(tag("CLASS")),
                    macro_pin_port_class_encode,
                    ws(tag(";")),
                )),
                many1(port_geometry),
            )),
            ws(tag("END")),
        ),
    )(input)
}

fn obs_statement(input: &str) -> LefRes<&str, Vec<PortShape>> {
    context(
        "Macro Obstacle Statement",
        delimited(ws(tag("OBS")), many1(port_geometry), ws(tag("END"))),
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
// partial checking model
fn pin_antenna_statement(input: &str) -> LefRes<&str, MacroPinAntenna> {
    context(
        "Macro Pin Antenna Statement",
        tuple((
            tuple((
                opt(delimited(
                    ws(tag("ANTENNAPARTIALMETALAREA")),
                    float,
                    ws(tag(";")),
                )),
                opt(delimited(
                    ws(tag("ANTENNAPARTIALMETALSIDEAREA")),
                    float,
                    ws(tag(";")),
                )),
                opt(delimited(
                    ws(tag("ANTENNAPARTIALCUTAREA")),
                    float,
                    ws(tag(";")),
                )),
            )),
            opt(delimited(ws(tag("ANTENNADIFFAREA")), float, ws(tag(";")))),
            opt(delimited(
                ws(tag("ANTENNAMODEL")),
                antenna_model_encode,
                ws(tag(";")),
            )),
            opt(delimited(ws(tag("ANTENNAGATEAREA")), float, ws(tag(";")))),
            tuple((
                opt(delimited(ws(tag("ANTENNAMAXAREACAR")), float, ws(tag(";")))),
                opt(delimited(
                    ws(tag("ANTENNAMAXSIDEAREACAR")),
                    float,
                    ws(tag(";")),
                )),
                opt(delimited(ws(tag("ANTENNAMAXCUTCAR")), float, ws(tag(";")))),
            )),
        )),
    )(input)
    .map(|(res, data)| {
        (
            res,
            MacroPinAntenna {
                partial_metal_area: (data.0).0,
                partial_metal_sidearea: (data.0).1,
                partial_cutarea: (data.0).2,
                diffarea: data.1,
                model: data.2.map_or(0, |x| x),
                gatearea: data.3,
                max_area_car: (data.4).0,
                max_sidearea_car: (data.4).1,
                max_cut_car: (data.4).2,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pin() {
        let test_str = "  PIN VDD
    DIRECTION INOUT ;
    USE POWER ;
    SHAPE ABUTMENT ;
    PORT
      LAYER M2 ;
        RECT 0 1.175 15.12 1.345 ;
        RECT 7.055 0.664 7.155 0.764 ;
        RECT 6.134 0.679 7.155 0.749 ;
        RECT 6.119 0.467 6.219 0.567 ;
        RECT 6.134 0.444 6.204 1.345 ;
      LAYER M1 ;
        RECT 7.055 0.679 7.815 0.749 ;
        RECT 7.055 0.664 7.155 0.764 ;
        RECT 6.108 1.212 6.99 1.302 ;
        RECT 6.87 1.012 6.99 1.302 ;
        RECT 6.357 1.21 6.747 1.32 ;
        RECT 6.488 1.012 6.608 1.32 ;
        RECT 6.108 1.012 6.228 1.302 ;
        RECT 6.119 0.467 6.219 0.567 ;
        RECT 0.296 0.467 6.219 0.537 ;
        RECT 5.35 1.212 5.85 1.302 ;
        RECT 5.73 1.012 5.85 1.302 ;
        RECT 5.35 1.012 5.47 1.302 ;
        RECT 4.593 1.212 5.093 1.302 ;
        RECT 4.973 1.012 5.093 1.302 ;
        RECT 4.593 1.012 4.713 1.302 ;
        RECT 3.834 1.212 4.334 1.302 ;
        RECT 4.214 1.012 4.334 1.302 ;
        RECT 3.834 1.012 3.954 1.302 ;
        RECT 3.071 1.212 3.571 1.302 ;
        RECT 3.451 1.012 3.571 1.302 ;
        RECT 3.071 1.012 3.191 1.302 ;
        RECT 2.314 1.212 2.814 1.302 ;
        RECT 2.694 1.012 2.814 1.302 ;
        RECT 2.314 1.012 2.434 1.302 ;
        RECT 1.551 1.212 2.051 1.302 ;
        RECT 1.931 1.012 2.051 1.302 ;
        RECT 1.551 1.012 1.671 1.302 ;
        RECT 0.792 1.212 1.292 1.302 ;
        RECT 1.172 1.012 1.292 1.302 ;
        RECT 0.792 1.012 0.912 1.302 ;
        RECT 0.029 1.212 0.529 1.302 ;
        RECT 0.409 1.012 0.529 1.302 ;
        RECT 0.029 0.867 0.149 1.302 ;
    END
      END VDD";
        let (_, _) = pin_statement(test_str).unwrap();
    }

    #[test]
    fn test_macro() {
        let test_str = "MACRO A2SDFFQN_X0P5M_A9TL40
  CLASS CORE ;
  ORIGIN 0 0 ;
  FOREIGN A2SDFFQN_X0P5M_A9TL40 0 0 ;
  SIZE 4.37 BY 1.26 ;
  SYMMETRY X Y ;
  SITE sc9mc_logic0040ll ;
  PIN A
    DIRECTION INPUT ;
    USE SIGNAL ;
    ANTENNAMODEL OXIDE1 ;
      ANTENNAGATEAREA 0.02 LAYER M1 ;
    PORT
      LAYER M1 ;
        RECT 1.385 0.595 1.515 0.805 ;
    END
  END A
  PIN B
    DIRECTION INPUT ;
    USE SIGNAL ;
    ANTENNAMODEL OXIDE1 ;
      ANTENNAGATEAREA 0.02 LAYER M1 ;
    PORT
      LAYER M1 ;
        RECT 1.04 0.595 1.17 0.805 ;
    END
  END B
  END A2SDFFQN_X0P5M_A9TL40";
        let (_, _) = macro_parser(test_str).unwrap();
    }
}
