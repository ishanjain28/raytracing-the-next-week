use crate::{
    types::{Ray, Vec3},
    Aabb, HitRecord, Hitable, Material,
};

pub struct MovingSphere<T: Material + Sized> {
    radius: f64,
    center_start: Vec3,
    center_end: Vec3,
    time_start: f64,
    time_end: f64,
    material: T,
}

impl<T: Material + Sized> MovingSphere<T> {
    pub fn new(
        center_start: Vec3,
        center_end: Vec3,
        time_start: f64,
        time_end: f64,
        radius: f64,
        material: T,
    ) -> Self {
        Self {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center_start
            + (self.center_end - self.center_start)
                * ((time - self.time_start) / (self.time_end - self.time_start))
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

impl<T: Material + Sized> Hitable for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        let discriminant_root = discriminant.sqrt();

        if discriminant > 0.0 {
            let root = (-b - discriminant_root) / a;
            if root < t_max && root > t_min {
                let p = ray.point_at_parameter(root);
                let normal = (p - self.center(ray.time())) / self.radius;
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
                let normal = (p - self.center(ray.time())) / self.radius;
                let (u, v) = Self::get_uv(normal);

                return Some(HitRecord {
                    t: root,
                    p,
                    u,
                    v,
                    normal: (p - self.center(ray.time())) / self.radius,
                    material: &self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let box_smol = Aabb::new(self.center(t0) - radius, self.center(t0) + radius);
        let box_big = Aabb::new(self.center(t1) - radius, self.center(t1) + radius);

        Some(Aabb::surrounding_box(box_smol, box_big))
    }
}
