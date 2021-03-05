use rand::Rng;

use crate::{texture::Perlin, types::Vec3, Texture};

pub struct PerlinNoise {
    noise: Perlin,
}

impl PerlinNoise {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            noise: Perlin::new(rng),
        }
    }
}

impl Texture for PerlinNoise {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
