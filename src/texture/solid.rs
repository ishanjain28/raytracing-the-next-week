use crate::{types::Vec3, Texture};

#[derive(Copy, Clone)]
pub struct Solid {
    color: Vec3,
}

impl Solid {
    pub const fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}
