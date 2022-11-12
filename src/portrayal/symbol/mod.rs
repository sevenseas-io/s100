mod area_symbol_placement;
pub use area_symbol_placement::AreaSymbolPlacement;

mod line_placement_mode;
pub use line_placement_mode::LinePlacementMode;

mod line_symbol_placement;
pub use line_symbol_placement::LineSymbolPlacement;

use crate::portrayal::{CRSType, Color, Vector};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    area_placement: Option<AreaSymbolPlacement>,
    line_placement: Option<LineSymbolPlacement>,
    offset: Vector,
    override_all: Option<Color>,
    override_colors: Vec<Color>,
    reference: String,
    rotation: f64,
    rotation_csr: CRSType,
    scale_factor: f64,
}

impl Symbol {
    pub fn area_placement(&self) -> Option<&AreaSymbolPlacement> {
        self.area_placement.as_ref()
    }

    pub fn offset(&self) -> &Vector {
        &self.offset
    }

    pub fn line_placement(&self) -> Option<&LineSymbolPlacement> {
        self.line_placement.as_ref()
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }

    pub fn override_all(&self) -> Option<&Color> {
        self.override_all.as_ref()
    }

    pub fn override_colors(&self) -> &[Color] {
        &self.override_colors
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn rotation_csr(&self) -> CRSType {
        self.rotation_csr
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}
