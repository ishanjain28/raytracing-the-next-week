use rand::{prelude::SmallRng, Rng};

use crate::{
    materials::{reflect, refract, schlick},
    types::{Ray, Vec3},
    HitRecord, Material,
};

pub struct Dielectric {
    reflection_index: f64,
}

impl Dielectric {
    pub fn new(reflection_index: f64) -> Self {
        Self { reflection_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut SmallRng,
    ) -> (Vec3, Option<Ray>) {
        let reflected_ray = reflect(ray_in.direction(), hit_rec.normal);
        // Glass absorbs nothing! So, Attenuation is always going to be 1.0 for this
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction().dot(&hit_rec.normal) > 0.0
        {
            (
                -hit_rec.normal,
                self.reflection_index,
                (ray_in.direction().dot(&hit_rec.normal) * self.reflection_index)
                    / ray_in.direction().length(),
            )
        } else {
            (
                hit_rec.normal,
                1.0 / self.reflection_index,
                (-ray_in.direction().dot(&hit_rec.normal)) / ray_in.direction().length(),
            )
        };

        if let Some(refracted_ray) = refract(ray_in.direction(), outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.reflection_index);

            if rng.gen::<f64>() < reflect_prob {
                (
                    attenuation,
                    Some(Ray::new(hit_rec.p, reflected_ray, ray_in.time())),
                )
            } else {
                (
                    attenuation,
                    Some(Ray::new(hit_rec.p, refracted_ray, ray_in.time())),
                )
            }
        } else {
            (
                attenuation,
                Some(Ray::new(hit_rec.p, reflected_ray, ray_in.time())),
            )
        }
    }
}
