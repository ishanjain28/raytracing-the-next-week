use crate::{types::Vec3, Texture};

pub struct Checker<T: Texture> {
    odd: T,
    even: T,
}

impl<T: Texture> Checker<T> {
    pub fn new(even: T, odd: T) -> Self {
        Self { odd, even }
    }
}

impl<T: Texture> Texture for Checker<T> {
    fn value(&self, u: f64, v: f64, p: &mut Vec3) {
        let sine_wave = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());

        if sine_wave < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
