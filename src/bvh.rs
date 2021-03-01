use std::cmp::Ordering;

use rand::{prelude::SliceRandom, Rng};

use crate::{types::Ray, Aabb, HitRecord, Hitable};

pub struct BvhNode<T: Hitable> {
    bounding_box: Aabb,
    left: HitNode<T>,
    right: HitNode<T>,
}

impl<T: Hitable + Clone> BvhNode<T> {
    pub fn new<R: Rng + ?Sized>(rng: &mut R, objects: &mut [T], t0: f64, t1: f64) -> Self {
        let comparator = [
            Self::box_x_compare,
            Self::box_y_compare,
            Self::box_z_compare,
        ]
        .choose(rng)
        .unwrap();

        objects.sort_by(comparator);

        let (left, right) = match objects.len() {
            1 => (
                HitNode::Direct(objects[0].clone()),
                HitNode::Direct(objects[0].clone()),
            ),
            2 => match comparator(&objects[0], &objects[1]) {
                Ordering::Greater => (
                    HitNode::Direct(objects[1].clone()),
                    HitNode::Direct(objects[0].clone()),
                ),
                _ => (
                    HitNode::Direct(objects[0].clone()),
                    HitNode::Direct(objects[1].clone()),
                ),
            },

            n => {
                let (l, r) = objects.split_at_mut(n / 2);
                (
                    HitNode::Bvh(Box::new(BvhNode::new(rng, l, t0, t1))),
                    HitNode::Bvh(Box::new(BvhNode::new(rng, r, t0, t1))),
                )
            }
        };

        let left_box = left
            .bounding_box(t0, t1)
            .expect("missing bounding box for left BVH Node");
        let right_box = right
            .bounding_box(t0, t1)
            .expect("missing bounding box for right BVH Node");

        Self {
            left,
            right,
            bounding_box: Aabb::surrounding_box(left_box, right_box),
        }
    }

    fn box_x_compare(obj1: &T, obj2: &T) -> Ordering {
        if let (Some(bbox_a), Some(bbox_b)) =
            (obj1.bounding_box(0.0, 0.0), obj2.bounding_box(0.0, 0.0))
        {
            return bbox_a.min.x().partial_cmp(&bbox_b.min.x()).unwrap();
        }

        panic!("No bounding box for this BVH Node!!")
    }

    fn box_y_compare(obj1: &T, obj2: &T) -> Ordering {
        if let (Some(bbox_a), Some(bbox_b)) =
            (obj1.bounding_box(0.0, 0.0), obj2.bounding_box(0.0, 0.0))
        {
            return bbox_a.min.y().partial_cmp(&bbox_b.min.y()).unwrap();
        }

        panic!("No bounding box for this BVH Node!!")
    }

    fn box_z_compare(obj1: &T, obj2: &T) -> Ordering {
        if let (Some(bbox_a), Some(bbox_b)) =
            (obj1.bounding_box(0.0, 0.0), obj2.bounding_box(0.0, 0.0))
        {
            return bbox_a.min.z().partial_cmp(&bbox_b.min.z()).unwrap();
        }

        panic!("No bounding box for this BVH Node!!")
    }
}

impl<T: Hitable> Hitable for BvhNode<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bounding_box(t_min, t_max)?;

        let hbox_left = self.left.hit(ray, t_min, t_max);

        let hbox_right = if let Some(ref hleft) = hbox_left {
            self.right.hit(ray, t_min, hleft.t)
        } else {
            self.right.hit(ray, t_min, t_max)
        };

        hbox_right.or(hbox_left)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

enum HitNode<T: Hitable> {
    Bvh(Box<BvhNode<T>>),
    Direct(T),
}

impl<T: Hitable> HitNode<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HitNode::Bvh(node) => node.hit(ray, t_min, t_max),
            HitNode::Direct(node) => node.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        match self {
            HitNode::Bvh(node) => node.bounding_box(t0, t1),
            HitNode::Direct(node) => node.bounding_box(t0, t1),
        }
    }
}
