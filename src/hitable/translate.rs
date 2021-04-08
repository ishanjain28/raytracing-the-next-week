use crate::{
    hitable::{HitRecord, Hitable},
    types::{Ray, Vec3},
    Aabb,
};

pub struct Translate<T> {
    object: T,
    offset: Vec3,
}

impl<T> Translate<T> {
    pub const fn new(object: T, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time());

        if let Some(mut hit) = self.object.hit(&moved_ray, t_min, t_max) {
            hit.p += self.offset;
            hit.set_face_normal(&moved_ray);

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.object
            .bounding_box(t0, t1)
            .map(|bbox| Aabb::new(bbox.min + self.offset, bbox.max + self.offset))
    }
}
