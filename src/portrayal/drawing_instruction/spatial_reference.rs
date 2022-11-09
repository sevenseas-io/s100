#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReference {
    forward: bool,
    reference: String,
}

impl SpatialReference {
    pub fn forward(&self) -> bool {
        self.forward
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }
}
