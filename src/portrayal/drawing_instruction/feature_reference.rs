#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureReference {
    reference: String,
}

impl FeatureReference {
    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }
}
