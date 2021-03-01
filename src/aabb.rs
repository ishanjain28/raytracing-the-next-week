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

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..=2 {
            let inverse_dir = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * inverse_dir;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inverse_dir;
            if inverse_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Self {
        let smol_box = Vec3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        );

        let big_box = Vec3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        );

        Self {
            min: smol_box,
            max: big_box,
        }
    }
}
