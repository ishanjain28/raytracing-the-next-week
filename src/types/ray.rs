use crate::types::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f64) -> Ray {
        Ray { a, b, time }
    }
    #[inline]
    pub const fn origin(&self) -> Vec3 {
        self.a
    }
    #[inline]
    pub const fn direction(&self) -> Vec3 {
        self.b
    }
    #[inline]
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
    #[inline]
    pub const fn time(&self) -> f64 {
        self.time
    }
}
