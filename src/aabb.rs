use crate::types::{Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let min = (self.min - ray.origin) / ray.direction;
        let max = (self.max - ray.origin) / ray.direction;

        let mins = min.min(max);
        let maxs = min.max(max);

        let tmin = mins.max_element(t_min);
        let tmax = maxs.min_element(t_max);

        tmax > tmin
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Self {
        let smol_box = Vec3::min(box0.min, box1.min);
        let big_box = Vec3::max(box0.max, box1.max);

        Self {
            min: smol_box,
            max: big_box,
        }
    }
}
