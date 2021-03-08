pub struct LefData {
    pub version: f32,
    pub dividechar: String,
    pub busbitchar: String,
    pub site: Option<LefSite>,
    pub macro_: Vec<LefMacro>,
}

pub struct LefSite {
    pub site_name: String,
    // site_class: PAD/CORE. (true/false)
    pub site_class: bool,
    // X|Y|R90 == 0|1|2
    pub site_symmetry: Option<Vec<u8>>,
    // row_pattern: previous_row_name, orient_code
    pub row_pattern: Option<Vec<(String, u8)>>,
    pub site_size: (u32, u32), // width ,height
}

pub struct LefMacro {
    pub macro_name: String,
    pub macro_class: u8,
    pub foreign_cell: (String, Option<(f32, f32)>, Option<u8>),
    pub origin: (f32, f32),
    pub eeq_macro: Option<String>,
    pub macro_size: (f32, f32),
    pub macro_symmetry: Vec<u8>,
    pub macro_site: Vec<MacroSite>,
    pub macro_pin: Vec<MacroPin>,
    pub macro_obs: Vec<PortShape>,
    pub macro_density: Option<MacroDensity>,
}

pub struct MacroSite {
    pub site_name: String,
    pub site_pattern: Option<String>,
}

pub struct MacroPin {
    pub pin_name: String,
    pub taper_rule: Option<String>,
    pub direction: u8,
    pub use_type: u8,
    pub net_expr: Option<String>,
    pub ground_sensitivity: Option<String>,
    pub supply_sensitivity: Option<String>,
    pub mustjoin: Option<String>,
    pub shape: Option<u8>,
    pub pin_port: PortShape, // (class,MacroPortObj) // assume only one port in each pin
    pub pin_antenna: Option<MacroPinAntenna>,
}

pub struct MacroPinAntenna {
    pub partial_metal_area: Option<f32>,
    pub partial_metal_sidearea: Option<f32>,
    pub partial_cutarea: Option<f32>,
    pub diffarea: Option<f32>,
    pub model: u8,
    pub gatearea: Option<f32>,
    pub max_area_car: Option<f32>,
    pub max_sidearea_car: Option<f32>,
    pub max_cut_car: Option<f32>,
}
pub struct PortShape {
    pub layer_name: String, // layer name
    pub geometries: Vec<PortGeometry>,
}

// pub struct PortLayerGeometry {
//     pub layer_name: String,
//     pub if_exceptpgnet: bool,
//     pub minspacing: Option<(bool, u32)>, //(if from spacing or designrulewidth, minspacing)
//     pub geometries: Vec<PortGeometry>,
// }

// pub struct PortViaGeometry {
//     pub via_name: String,
//     pub via_location: (f32, f32),
// }

pub enum PortGeometry {
    Path(Vec<(f32, f32)>),
    Rect(((f32, f32), (f32, f32))),
    Polygon(Vec<(f32, f32)>),
    Via((String, (f32, f32))),
}

// pub enum MacroOBS {
//     LayerObj(PortLayerGeometry),
//     ViaObj(PortViaGeometry),
// }

pub struct MacroDensity {
    pub layer_name: String,
    pub rect_region: Vec<(((f32, f32), (f32, f32)), f32)>,
}
