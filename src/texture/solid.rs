use crate::{types::Vec3, Texture};

pub struct Solid {
    color: Vec3,
}

impl Solid {
    pub const fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, p: &mut Vec3) {
        *p = self.color
    }
}