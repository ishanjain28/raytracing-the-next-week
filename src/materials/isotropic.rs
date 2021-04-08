use rand::prelude::SmallRng;

use crate::{
    hitable::HitRecord,
    materials::random_point_in_unit_sphere,
    types::{Ray, Vec3},
    Material, Texture,
};

pub struct Isotropic<T> {
    texture: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Self { texture }
    }
}

impl<T: Texture + Send + Sync> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord, rng: &mut SmallRng) -> (Vec3, Option<Ray>) {
        (
            self.texture.value(hit_rec.u, hit_rec.v, hit_rec.p),
            Some(Ray::new(
                hit_rec.p,
                random_point_in_unit_sphere(rng),
                ray.time(),
            )),
        )
    }
}
