use crate::types::{HitRecord, Hitable, Material, Ray, Vec3};

pub struct MovingSphere {
    radius: f32,
    center_start: Vec3,
    center_end: Vec3,
    time_start: f32,
    time_end: f32,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center_start: Vec3,
        center_end: Vec3,
        time_start: f32,
        time_end: f32,
        radius: f32,
        material: Box<dyn Material>,
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

    fn center(&self, time: f32) -> Vec3 {
        self.center_start
            + (self.center_end - self.center_start)
                * ((time - self.time_start) / (self.time_end - self.time_start))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
}
