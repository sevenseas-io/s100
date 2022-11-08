#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    token: String,
    transparency: f64,
}

impl Color {
    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }
}
