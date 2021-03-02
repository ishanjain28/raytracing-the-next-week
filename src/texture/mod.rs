mod solid;

pub use solid::Solid;

use crate::types::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64) -> Vec3;
}
