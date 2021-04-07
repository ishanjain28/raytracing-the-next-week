mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;
use rand::{prelude::SmallRng, Rng};

use crate::{
    hitable::HitRecord,
    types::{Ray, Vec3},
};

pub trait Material: Send + Sync {
    // scatter returns the attenuation and the scattered ray.
    // Attenuation is ignored completely if there is no scattered ray
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_rec: &HitRecord,
        _rng: &mut SmallRng,
    ) -> (Vec3, Option<Ray>) {
        (Vec3::splat(0.0), None)
    }

    fn emit(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::splat(0.0)
    }
}

// Christophe Schlick's Polynomial approximation to figure out reflectivity as the angle changes
// See Fresnel Equations, https://en.wikipedia.org/wiki/Fresnel_equations
fn schlick(cosine: f64, reflection_index: f64) -> f64 {
    let mut r0 = (1.0 - reflection_index) / (1.0 + reflection_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * incident.dot(&normal) * 2.0
}

// Snell's Law
fn refract(incident: Vec3, normal: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = incident.unit_vector();
    let dt = uv.dot(&normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
    } else {
        None
    }
}

fn random_point_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
    let mut point = Vec3::random(rng) * 2.0 - Vec3::splat(1.0);
    while point.sq_len() >= 1.0 {
        point = Vec3::random(rng) * 2.0 - Vec3::splat(1.0);
    }
    point
}

pub trait MaterialBuilder<T> {
    type Finished;

    fn material(self, material: T) -> Self::Finished;
}
