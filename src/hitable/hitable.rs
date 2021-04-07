use std::sync::Arc;

use crate::{
    types::{Ray, Vec3},
    Aabb, Material,
};

pub struct HitRecord<'a> {
    ///  Rays are represented by A + t * B
    ///  where A is the source point and B destination point
    ///  by adjusting t we can move forward/back on the ray
    ///
    ///  t is the point at which a ray intersected another object.
    ///  As in, If we put this value of t in A + t * B equation, We'll get the exact
    ///  point at which a ray intersects some other object
    pub t: f64,
    /// Ray object otherwise is represented by the Source/Destination points
    /// p is what we get when we perform the operation, A + t * B
    /// i.e. A vector from Ray source to the point t
    pub p: Vec3,

    /// unit outward facing normal
    pub normal: Vec3,

    /// material if any of the surface
    pub material: &'a dyn Material,

    /// texture coordinates for an object
    pub u: f64,
    pub v: f64,

    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        p: Vec3,
        normal: Vec3,
        material: &'a dyn Material,
        (u, v): (f64, f64),
    ) -> Self {
        Self {
            t,
            p,
            normal,
            material,
            u,
            v,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = ray.direction.dot(&self.normal) < 0.0;

        self.normal = if self.front_face {
            self.normal
        } else {
            -self.normal
        }
    }
}

pub trait Hitable {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb>;
}

impl<T: Hitable + ?Sized> Hitable for Arc<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.as_ref().bounding_box(t0, t1)
    }
}

pub struct Translate<T> {
    object: T,
    offset: Vec3,
}

impl<T> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
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
        if let Some(bbox) = self.object.bounding_box(t0, t1) {
            Some(Aabb::new(bbox.min + self.offset, bbox.max + self.offset))
        } else {
            None
        }
    }
}
