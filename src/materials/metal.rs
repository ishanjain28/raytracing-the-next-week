use rand::prelude::SmallRng;

use crate::{
    hitable::HitRecord,
    materials::{random_point_in_unit_sphere, reflect},
    types::{Ray, Vec3},
    Material,
};

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo, fuzz: 0.0 }
    }
    pub fn with_fuzz(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut SmallRng,
    ) -> (Vec3, Option<Ray>) {
        let reflected_ray = reflect(ray_in.direction.unit_vector(), hit_rec.normal);
        let scattered_ray = Ray::new(
            hit_rec.p,
            reflected_ray + random_point_in_unit_sphere(rng) * self.fuzz,
            ray_in.time(),
        );

        if scattered_ray.direction.dot(&hit_rec.normal) > 0.0 {
            (self.albedo, Some(scattered_ray))
        } else {
            (self.albedo, None)
        }
    }
}
