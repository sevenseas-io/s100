#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
