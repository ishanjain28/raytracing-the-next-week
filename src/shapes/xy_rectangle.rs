use crate::{
    types::{Ray, Vec3},
    Aabb, HitRecord, Hitable, Material,
};

pub struct XyRectangle<T: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: T,
}

impl<T: Material> XyRectangle<T> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: T) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl<T: Material> Hitable for XyRectangle<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();

        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin().x() + t * ray.direction().x();
            let y = ray.origin().y() + t * ray.direction().y();

            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (y - self.y0) / (self.y1 - self.y0);

                Some(HitRecord::new(
                    t,
                    ray.point_at_parameter(t),
                    Vec3::new(0.0, 0.0, 1.0),
                    &self.material,
                    (u, v),
                ))
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // Since this is a axis aligned Rectangle and we are using AABB BVH, Gap between the rectangle and
        // the bounding box will be infinitely small

        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
