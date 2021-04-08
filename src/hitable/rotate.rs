use std::marker::PhantomData;

use crate::{
    hitable::{HitRecord, Hitable},
    types::{Ray, Vec3},
    Aabb, Dimension, X, Y, Z,
};

pub struct Rotate<D1, D2, D3, T: Hitable> {
    hitable: T,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,

    _tag: PhantomData<(D1, D2, D3)>,
}

impl<D1, D2, D3, T> Rotate<D1, D2, D3, T>
where
    D1: Dimension,
    D2: Dimension,
    D3: Dimension,
    T: Hitable,
{
    pub fn new(object: T, angle: f64) -> Rotate<D1, D2, D3, T> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Vec3::splat(f64::MAX);
        let mut max = Vec3::splat(-f64::MAX);

        let bbox = if let Some(bbox) = object.bounding_box(0.0, 1.0) {
            for i in 0..2 {
                let i = i as f64;
                for j in 0..2 {
                    let j = j as f64;
                    for k in 0..2 {
                        let k = k as f64;

                        // D1 will be the axis about which we are rotating
                        let d1 = i * bbox.max.get::<D1>() + (1.0 - i) * bbox.min.get::<D1>();

                        let d2 = j * bbox.max.get::<D2>() + (1.0 - j) * bbox.min.get::<D2>();
                        let d3 = k * bbox.max.get::<D3>() + (1.0 - k) * bbox.min.get::<D3>();

                        let new_d2 = cos_theta * d2 + sin_theta * d3;
                        let new_d3 = -sin_theta * d2 + cos_theta * d3;

                        let tester = Vec3::splat(0.0)
                            .set::<D1>(d1)
                            .set::<D2>(new_d2)
                            .set::<D3>(new_d3);

                        min = Vec3::min(tester, min);
                        max = Vec3::max(tester, max);
                    }
                }
            }

            Aabb::new(min, max)
        } else {
            Aabb::new(min, max)
        };

        Rotate {
            hitable: object,
            sin_theta,
            cos_theta,
            bbox: Some(bbox),
            _tag: PhantomData,
        }
    }
}

impl<D1, D2, D3, T> Hitable for Rotate<D1, D2, D3, T>
where
    D1: Dimension,
    D2: Dimension,
    D3: Dimension,
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = ray
            .origin
            .set::<D2>(
                self.cos_theta * ray.origin.get::<D2>() - self.sin_theta * ray.origin.get::<D3>(),
            )
            .set::<D3>(
                self.sin_theta * ray.origin.get::<D2>() + self.cos_theta * ray.origin.get::<D3>(),
            );

        let direction = ray
            .direction
            .set::<D2>(
                self.cos_theta * ray.direction.get::<D2>()
                    - self.sin_theta * ray.direction.get::<D3>(),
            )
            .set::<D3>(
                self.sin_theta * ray.direction.get::<D2>()
                    + self.cos_theta * ray.direction.get::<D3>(),
            );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        let mut hit = self.hitable.hit(&rotated_ray, t_min, t_max)?;

        hit.p = hit
            .p
            .set::<D2>(self.cos_theta * hit.p.get::<D2>() + self.sin_theta * hit.p.get::<D3>())
            .set::<D3>(-self.sin_theta * hit.p.get::<D2>() + self.cos_theta * hit.p.get::<D3>());

        hit.normal = hit
            .normal
            .set::<D2>(
                self.cos_theta * hit.normal.get::<D2>() + self.sin_theta * hit.normal.get::<D3>(),
            )
            .set::<D3>(
                -self.sin_theta * hit.normal.get::<D2>() + self.cos_theta * hit.normal.get::<D3>(),
            );

        hit.set_face_normal(&rotated_ray);

        Some(hit)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        self.bbox
    }
}
