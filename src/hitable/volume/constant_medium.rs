use crate::{
    hitable::{HitRecord, Hitable},
    types::{Ray, Vec3},
    Aabb, Material,
};

pub struct ConstantMedium<A: Hitable, B: Material> {
    neg_inv_density: f64,
    boundary: A,
    phase_function: B,
}

impl<A: Hitable, B: Material> ConstantMedium<A, B> {
    pub fn new(boundary: A, phase_function: B, d: f64) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<A: Hitable, B: Material> Hitable for ConstantMedium<A, B> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit1 = self.boundary.hit(ray, f64::MIN, f64::MAX)?;
        let mut hit2 = self.boundary.hit(ray, hit1.t + 0.0001, f64::MAX)?;

        hit1.t = hit1.t.max(t_min);
        hit2.t = hit2.t.min(t_max);

        if hit1.t >= hit2.t {
            return None;
        };

        hit1.t = hit1.t.max(0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;

        Some(HitRecord {
            t,
            p: ray.point_at_parameter(t),
            material: &self.phase_function,
            u: 0.0,
            v: 0.0,

            // Arbitrary
            front_face: true,
            normal: Vec3::new(1.0, 0.0, 0.0),
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}
