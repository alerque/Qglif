// SVG icons
// This file is automatically generated! See icons/gen.sh !
pub type SvgImageData = (u32, u32, Vec<u8>);

use nsvg;

use crate::STATE;

fn parse(name: &str, str: &'static str) -> SvgImageData {
    let dpi = STATE.with(|v| v.borrow().dpi) * 96.;
    debug!("Building SVG icons with DPI {}", dpi);
    nsvg::parse_str(str, nsvg::Units::Pixel, dpi as f32)
        .expect(&format!("Failed to parse SVG {}", name))
        .rasterize_to_raw_rgba(0.75)
        .expect(&format!("Failed to rasterize {}", name))
}

const KNIFE_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/knife.svg"));
lazy_static! {
    pub static ref KNIFE_ICON_IMAGE: SvgImageData = parse(stringify!(knife), KNIFE_ICON);
}

const MEASURE_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/measure.svg"));
lazy_static! {
    pub static ref MEASURE_ICON_IMAGE: SvgImageData = parse(stringify!(measure), MEASURE_ICON);
}

const PAN_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/pan.svg"));
lazy_static! {
    pub static ref PAN_ICON_IMAGE: SvgImageData = parse(stringify!(pan), PAN_ICON);
}

const PEN_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/pen.svg"));
lazy_static! {
    pub static ref PEN_ICON_IMAGE: SvgImageData = parse(stringify!(pen), PEN_ICON);
}

const PENCIL_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/pencil.svg"));
lazy_static! {
    pub static ref PENCIL_ICON_IMAGE: SvgImageData = parse(stringify!(pencil), PENCIL_ICON);
}

const SELECT_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/select.svg"));
lazy_static! {
    pub static ref SELECT_ICON_IMAGE: SvgImageData = parse(stringify!(select), SELECT_ICON);
}

const SHAPES_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/shapes.svg"));
lazy_static! {
    pub static ref SHAPES_ICON_IMAGE: SvgImageData = parse(stringify!(shapes), SHAPES_ICON);
}

const TEXT_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/text.svg"));
lazy_static! {
    pub static ref TEXT_ICON_IMAGE: SvgImageData = parse(stringify!(text), TEXT_ICON);
}

const ZOOM_ICON: &str = include_str!(concat!(env!("PWD"), "/", "resources/icons/zoom.svg"));
lazy_static! {
    pub static ref ZOOM_ICON_IMAGE: SvgImageData = parse(stringify!(zoom), ZOOM_ICON);
}
