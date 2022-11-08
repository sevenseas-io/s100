use crate::portrayal::symbol::Symbol;

use super::{DrawingInstruction, FeatureReference, SpatialReference};

#[derive(Clone, Debug, PartialEq)]
pub struct PointInstruction {
    viewing_group: String,
    display_plane: String,
    drawing_priority: i64,
    scale_minimum: Option<i64>,
    scale_maximum: Option<i64>,
    feature_reference: FeatureReference,
    spatial_reference: Vec<SpatialReference>,
    symbol: Symbol,
}

impl PointInstruction {
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl DrawingInstruction for PointInstruction {
    fn viewing_group(&self) -> &str {
        self.viewing_group.as_str()
    }

    fn display_plane(&self) -> &str {
        self.display_plane.as_str()
    }

    fn drawing_priority(&self) -> i64 {
        self.drawing_priority
    }

    fn scale_minimum(&self) -> Option<i64> {
        self.scale_minimum
    }

    fn scale_maximum(&self) -> Option<i64> {
        self.scale_maximum
    }

    fn feature_reference(&self) -> &FeatureReference {
        &self.feature_reference
    }

    fn spatial_reference(&self) -> &[SpatialReference] {
        &self.spatial_reference[..]
    }
}
