use std::sync::Arc;

use crate::{
    hitable::{hitable_list::HitableList, shapes::RectBuilder, HitRecord, Hitable},
    materials::{Material, MaterialBuilder},
    types::{Ray, Vec3},
    Aabb,
};

pub struct Cuboid {
    min: Vec3,
    max: Vec3,
    sides: HitableList,
}

impl Cuboid {
    pub fn new(p0: Vec3, p1: Vec3, mat: impl Material + Clone + 'static) -> Self {
        Self {
            min: p0,
            max: p1,
            sides: Self::build_cuboid(p0, p1, mat),
        }
    }

    fn build_cuboid(p0: Vec3, p1: Vec3, mat: impl Material + Clone + 'static) -> HitableList {
        let mut sides = HitableList {
            list: Vec::with_capacity(6),
        };

        sides.push(Arc::new(
            RectBuilder
                .x(p0.x()..=p1.x())
                .y(p0.y()..=p1.y())
                .z(p1.z())
                .material(mat.clone()),
        ));
        sides.push(Arc::new(
            RectBuilder
                .x(p0.x()..=p1.x())
                .y(p0.y()..=p1.y())
                .z(p0.z())
                .material(mat.clone()),
        ));

        sides.push(Arc::new(
            RectBuilder
                .x(p0.x()..=p1.x())
                .z(p0.z()..=p1.z())
                .y(p1.y())
                .material(mat.clone()),
        ));
        sides.push(Arc::new(
            RectBuilder
                .x(p0.x()..=p1.x())
                .z(p0.z()..=p1.z())
                .y(p0.y())
                .material(mat.clone()),
        ));

        sides.push(Arc::new(
            RectBuilder
                .y(p0.y()..=p1.y())
                .z(p0.z()..=p1.z())
                .x(p1.x())
                .material(mat.clone()),
        ));
        sides.push(Arc::new(
            RectBuilder
                .y(p0.y()..=p1.y())
                .z(p0.z()..=p1.z())
                .x(p0.x())
                .material(mat),
        ));

        sides
    }
}

impl Hitable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
