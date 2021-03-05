use rand::prelude::SmallRng;

use crate::{
    materials::random_point_in_unit_sphere,
    types::{Ray, Vec3},
    HitRecord, Material, Texture,
};

pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture + Send + Sync> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture + Send + Sync> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord, rng: &mut SmallRng) -> (Vec3, Option<Ray>) {
        let scatter_direction = hit_rec.normal + random_point_in_unit_sphere(rng);
        let scattered_ray = Ray::new(hit_rec.p, scatter_direction, ray.time());

        (
            self.albedo.value(hit_rec.u, hit_rec.v, hit_rec.p),
            Some(scattered_ray),
        )
    }
}
