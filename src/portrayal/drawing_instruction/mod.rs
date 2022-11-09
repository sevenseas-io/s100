mod feature_reference;
pub use feature_reference::FeatureReference;

mod point_instruction;
pub use point_instruction::PointInstruction;

mod spatial_reference;
pub use spatial_reference::SpatialReference;

pub trait DrawingInstruction {
    fn viewing_group(&self) -> &str;
    fn display_plane(&self) -> &str;
    fn drawing_priority(&self) -> i64;
    fn scale_minimum(&self) -> Option<i64>;
    fn scale_maximum(&self) -> Option<i64>;
    fn feature_reference(&self) -> &FeatureReference;
    fn spatial_reference(&self) -> &[SpatialReference];
}
