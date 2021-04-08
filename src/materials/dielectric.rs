use rand::{prelude::SmallRng, Rng};

use crate::{
    hitable::HitRecord,
    materials::{reflect, refract, schlick},
    types::{Ray, Vec3},
    Material,
};

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut SmallRng,
    ) -> (Vec3, Option<Ray>) {
        // Glass absorbs nothing! So, Attenuation is always going to be 1.0 for this
        let attenuation = Vec3::splat(1.0);

        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cosine = (-unit_direction).dot(&hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cosine * cosine).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || schlick(cosine, refraction_ratio) > rng.gen::<f64>() {
            let direction = reflect(unit_direction, hit_rec.normal);
            (
                attenuation,
                Some(Ray::new(hit_rec.p, direction, ray_in.time())),
            )
        } else if let Some(direction) = refract(unit_direction, hit_rec.normal, refraction_ratio) {
            (
                attenuation,
                Some(Ray::new(hit_rec.p, direction, ray_in.time())),
            )
        } else {
            let direction = reflect(unit_direction, hit_rec.normal);
            (
                attenuation,
                Some(Ray::new(hit_rec.p, direction, ray_in.time())),
            )
        }
    }
}
