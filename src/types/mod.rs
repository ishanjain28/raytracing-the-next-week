mod color;
mod dimension;
mod ray;

pub use color::Color;
pub use dimension::{Dimension, X, Y, Z};
pub use ray::Ray;

#[cfg(not(target_arch = "x86_64"))]
mod vec3;
#[cfg(not(target_arch = "x86_64"))]
pub use vec3::Vec3;

#[cfg(target_arch = "x86_64")]
mod simd_vec3;
#[cfg(target_arch = "x86_64")]
pub use simd_vec3::Vec3;
