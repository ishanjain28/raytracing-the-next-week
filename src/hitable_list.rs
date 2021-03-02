use std::sync::Arc;

use crate::{demos::ParallelHit, types::Ray, Aabb, HitRecord, Hitable};

pub struct HitableList {
    pub list: Vec<Arc<dyn ParallelHit>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec: Option<HitRecord> = None;
        for obj in &self.list {
            if let Some(l_hit_rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = l_hit_rec.t;
                hit_rec = Some(l_hit_rec);
            }
        }
        hit_rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.list.is_empty() {
            return None;
        }

        let mut output_box = None;

        for obj in self.list.iter() {
            if let Some(bbox) = obj.bounding_box(t0, t1) {
                if let Some(ref mut opbox) = output_box {
                    *opbox = Aabb::surrounding_box(*opbox, bbox);
                } else {
                    output_box = Some(bbox);
                }
            } else {
                return output_box;
            }
        }

        output_box
    }
}

impl HitableList {
    pub fn push(&mut self, obj: Arc<dyn ParallelHit>) {
        self.list.push(obj);
    }
}
