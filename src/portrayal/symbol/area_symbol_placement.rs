use super::LinePlacementMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AreaSymbolPlacement {
    placement_mode: LinePlacementMode,
}

impl AreaSymbolPlacement {
    pub fn placement_mode(&self) -> LinePlacementMode {
        self.placement_mode
    }
}
