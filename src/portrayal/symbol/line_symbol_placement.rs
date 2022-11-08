use super::LinePlacementMode;

#[derive(Clone, Debug, PartialEq)]
pub struct LineSymbolPlacement {
    offset: f64,
    placement_mode: LinePlacementMode,
}

impl LineSymbolPlacement {
    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn placement_mode(&self) -> LinePlacementMode {
        self.placement_mode
    }
}
