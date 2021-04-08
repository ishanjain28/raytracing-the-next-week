pub mod bvh;
pub mod hitable_list;
mod rotate;
pub mod shapes;
mod translate;
pub mod volume;

pub use bvh::*;
pub use translate::*;

use std::sync::Arc;

use crate::{
    hitable::rotate::Rotate,
    types::{Ray, Vec3},
    Aabb, Material, X, Y, Z,
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

    fn translate(self, offset: impl Into<Vec3>) -> Translate<Self>
    where
        Self: Sized,
    {
        Translate::new(self, offset.into())
    }

    fn rotate_x(self, angle: f64) -> Rotate<X, Y, Z, Self>
    where
        Self: Sized,
    {
        Rotate::new(self, angle)
    }

    fn rotate_y(self, angle: f64) -> Rotate<Y, X, Z, Self>
    where
        Self: Sized,
    {
        Rotate::new(self, angle)
    }

    fn rotate_z(self, angle: f64) -> Rotate<Z, Y, X, Self>
    where
        Self: Sized,
    {
        Rotate::new(self, angle)
    }
}

impl<T: Hitable + ?Sized> Hitable for Arc<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.as_ref().bounding_box(t0, t1)
    }
}
