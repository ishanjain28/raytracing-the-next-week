use std::sync::Arc;

use crate::{
    types::{Material, Ray, Vec3},
    Aabb,
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
