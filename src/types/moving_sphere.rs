use crate::{
    types::{Material, Ray, Vec3},
    Aabb, HitRecord, Hitable,
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
                return Some(HitRecord {
                    t: root,
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    material: &self.material,
                });
            }
            let root = (-b + discriminant_root) / a;
            if root < t_max && root > t_min {
                let p = ray.point_at_parameter(root);
                return Some(HitRecord {
                    t: root,
                    p,
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
