mod checker;
mod image_texture;
mod perlin;
mod perlin_noise;
mod solid;

pub use checker::Checker;
pub use image_texture::ImageTexture;
pub use perlin::Perlin;
pub use perlin_noise::PerlinNoise;
pub use solid::Solid;

use crate::types::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
