use rand::Rng;

use crate::{texture::Perlin, types::Vec3, Texture};

pub struct PerlinNoise {
    noise: Perlin,
    scale: f64,
}

impl PerlinNoise {
    #[allow(dead_code)]
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale: 1.0,
        }
    }

    pub fn with_scale<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for PerlinNoise {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p * self.scale)
    }
}
