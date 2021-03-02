use crate::{
    types::{Ray, Vec3},
    Aabb, HitRecord, Hitable, Material,
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

    /// p is a point on the sphere of radius 1 & center at origin
    /// u is between [0,1]. Angle around Y axis from -X axis
    /// v is between [0,1]. Angle from -Y to +Y axis
    pub fn get_uv(p: Vec3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
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
                let normal = (p - self.center) / self.radius;
                let (u, v) = Self::get_uv(normal);

                return Some(HitRecord {
                    t: root,
                    p,
                    u,
                    v,
                    normal,
                    material: &self.material,
                });
            }

            let root = (-b + discriminant_root) / a;
            if root < t_max && root > t_min {
                let p = ray.point_at_parameter(root);
                let normal = (p - self.center) / self.radius;
                let (u, v) = Self::get_uv(normal);

                return Some(HitRecord {
                    t: root,
                    p,
                    u,
                    v,
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