use rand::prelude::SmallRng;

use crate::{types::Vec3, Hitable};

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

    pub fn color<T: Hitable>(
        &self,
        world: &T,
        rng: &mut SmallRng,
        background: &Vec3,
        depth: u32,
    ) -> Vec3 {
        if let Some(hit_rec) = world.hit(self, 0.001, std::f64::MAX) {
            if depth >= 50 {
                Vec3::new(0.0, 0.0, 0.0)
            } else {
                let material = hit_rec.material;
                let emitted_color = hit_rec.material.emit(hit_rec.u, hit_rec.v, hit_rec.p);

                if let (attenuation, Some(scattered_ray)) = material.scatter(self, &hit_rec, rng) {
                    emitted_color
                        + attenuation * scattered_ray.color(world, rng, background, depth + 1)
                } else {
                    emitted_color
                }
            }
        } else {
            *background
        }
    }
}
