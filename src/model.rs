pub struct LefData {
    pub version: f32,
    pub dividechar: String,
    pub busbitchar: String,
    pub site: LefSite,
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
    pub foreign_cell: (String, Option<(i32, i32)>, Option<u8>),
    pub origin: (i32, i32),
    pub eeq_macro: String,
    pub macro_size: (u32, u32),
    pub macro_symmetry: Vec<u8>,
    pub macro_site: Vec<MacroSite>,
    pub macro_pin: Vec<MacroPin>,
    pub macro_obs: Vec<MacroOBS>,
    pub macro_density: Vec<MacroDensity>,
}

pub struct MacroSite {
    pub site_name: String,
    pub site_pattern: Option<String>,
}

pub struct MacroPin {
    pub pin_name: String,
    pub taper_rule: String,
    pub direction: u8,
    pub use_type: u8,
    pub net_expr: String,
    pub ground_sensitivity: String,
    pub supply_sensitivity: String,
    pub mustjoin: String,
    pub shape: u8,
    pub macro_port: Vec<(u8, PortLayerGeometry)>, // (class,MacroPortObj)
}

pub struct PortLayerGeometry {
    pub layer_name: String,
    pub if_exceptpgnet: bool,
    pub minspacing: (bool, u32), //(if from spacing or designrulewidth, minspacing)
    pub geometries: Vec<PortGeometry>,
}

pub struct PortViaGeometry {
    pub via_name: String,
    pub via_location: (i32, i32),
}

pub enum PortGeometry {
    Path(Vec<(i32, i32)>),
    Rect(((i32, i32), (i32, i32))),
    Polygon(Vec<(i32, i32)>),
}

pub enum MacroOBS {
    LayerObj(PortLayerGeometry),
    ViaObj(PortViaGeometry),
}

pub struct MacroDensity {
    pub layer_name: String,
    pub rect_region: Vec<(((i32, i32), (i32, i32)), f32)>,
}
