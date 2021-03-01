use crate::{
    types::{Material, Ray, Vec3},
    Aabb, HitRecord, Hitable,
};

pub struct Sphere<T: Material + Sized> {
    center: Vec3,
    radius: f64,
    material: T,
}

impl<T: Material + Sized> Sphere<T> {
    pub fn new(center: Vec3, radius: f64, material: T) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material + Sized> Hitable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        // The discriminant is calculated using b^2 - 4 * a * c
        // but in this specific case, If we put the equation in the
        // formula to find quadratic roots, We can get this shorter
        // formula to find the discriminant.
        // Check this for detailed proof
        // https://vchizhov.github.io/resources/ray%20tracing/ray%20tracing%20tutorial%20series%20vchizhov/ray_casting/part1/intersecting_a_sphere.md.html#appendix
        let discriminant = b * b - a * c;
        let discriminant_root = discriminant.sqrt();

        if discriminant > 0.0 {
            let root = (-b - discriminant_root) / a;
            if root < t_max && root > t_min {
                let p = ray.point_at_parameter(root);
                return Some(HitRecord {
                    t: root,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }

            let root = (-b + discriminant_root) / a;
            if root < t_max && root > t_min {
                let p = ray.point_at_parameter(root);

                return Some(HitRecord {
                    t: root,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(Aabb::new(self.center - radius, self.center + radius))
    }
}
