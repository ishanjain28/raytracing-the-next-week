use std::{marker::PhantomData, ops::RangeInclusive};

use crate::{
    hitable::{HitRecord, Hitable},
    materials::MaterialBuilder,
    types::{Ray, Vec3},
    Aabb, Dimension, Material, X, Y, Z,
};

type DimRange = RangeInclusive<f64>;

pub struct Rectangle<D1, D2, D3, T> {
    d1_range: DimRange,
    d2_range: DimRange,
    d3: f64,
    material: T,
    tag: PhantomData<(D1, D2, D3)>,
}

impl<D1, D2, D3, T> Hitable for Rectangle<D1, D2, D3, T>
where
    T: Material,
    D1: Dimension,
    D2: Dimension,
    D3: Dimension,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.d3 - ray.origin.get::<D3>()) / ray.direction.get::<D3>();

        if t < t_min || t > t_max {
            return None;
        }

        let d1 = ray.origin.get::<D1>() + t * ray.direction.get::<D1>();
        let d2 = ray.origin.get::<D2>() + t * ray.direction.get::<D2>();

        if !self.d1_range.contains(&d1) || !self.d2_range.contains(&d2) {
            return None;
        }

        let u = (d1 - self.d1_range.start()) / (self.d1_range.end() - self.d1_range.start());
        let v = (d2 - self.d2_range.start()) / (self.d2_range.end() - self.d2_range.start());

        let mut hit_rec = HitRecord::new(
            t,
            ray.point_at_parameter(t),
            Vec3::splat(0.0).set::<D3>(1.0),
            &self.material,
            (u, v),
        );

        hit_rec.set_face_normal(ray);

        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // Since this is a axis aligned Rectangle and we are using AABB BVH, Gap between the rectangle and
        // the bounding box will be infinitely small

        let (&d1_0, &d1_1) = (self.d1_range.start(), self.d1_range.end());
        let (&d2_0, &d2_1) = (self.d2_range.start(), self.d2_range.end());

        let min = Vec3::splat(self.d3 - 0.0001)
            .set::<D1>(d1_0)
            .set::<D2>(d2_0);
        let max = Vec3::splat(self.d3 + 0.0001)
            .set::<D1>(d1_1)
            .set::<D2>(d2_1);

        Some(Aabb::new(min, max))
    }
}

// taken from, https://github.com/Globidev/toy-rt/blob/master/trt-core/src/hit/rect.rs#L74
// because it's amazing!

pub struct RectBuilder;

macro_rules! builder {
    ($name:ident, $dim:ty) => {
        pub fn $name(self, range: RangeInclusive<f64>) -> OneBoundedRectBuilder<$dim> {
            OneBoundedRectBuilder {
                range,
                tag: PhantomData,
            }
        }
    };
}

pub struct OneBoundedRectBuilder<D> {
    range: DimRange,
    tag: PhantomData<D>,
}

impl RectBuilder {
    builder!(x, X);
    builder!(y, Y);
    builder!(z, Z);
}

macro_rules! one_bounded_rect_builder {
    ($name:ident, $dim1: ty, $dim2: ty) => {
        pub fn $name(self, d2_range: DimRange) -> TwoBoundedRectBuilder<$dim1, $dim2> {
            TwoBoundedRectBuilder {
                d1_range: self.range,
                d2_range,
                tag: PhantomData,
            }
        }
    };
}

impl OneBoundedRectBuilder<X> {
    one_bounded_rect_builder!(y, X, Y);
    one_bounded_rect_builder!(z, X, Z);
}
impl OneBoundedRectBuilder<Y> {
    one_bounded_rect_builder!(x, Y, X);
    one_bounded_rect_builder!(z, Y, Z);
}
impl OneBoundedRectBuilder<Z> {
    one_bounded_rect_builder!(x, Z, X);
    one_bounded_rect_builder!(y, Z, Y);
}

pub struct TwoBoundedRectBuilder<D1, D2> {
    d1_range: DimRange,
    d2_range: DimRange,
    tag: PhantomData<(D1, D2)>,
}

macro_rules! two_bounded_rect_builder {
    ($name:ident, $dim1: ty, $dim2: ty, $dim3: ty) => {
        pub fn $name(self, $name: f64) -> ThreeBoundedRectBuilder<$dim1, $dim2, $dim3> {
            ThreeBoundedRectBuilder {
                d1_range: self.d1_range,
                d2_range: self.d2_range,
                d3: $name,
                tag: PhantomData,
            }
        }
    };
}

impl TwoBoundedRectBuilder<X, Y> {
    two_bounded_rect_builder!(z, X, Y, Z);
}
impl TwoBoundedRectBuilder<X, Z> {
    two_bounded_rect_builder!(y, X, Z, Y);
}
impl TwoBoundedRectBuilder<Y, Z> {
    two_bounded_rect_builder!(x, Y, Z, X);
}

pub struct ThreeBoundedRectBuilder<D1, D2, D3> {
    d1_range: DimRange,
    d2_range: DimRange,
    d3: f64,
    tag: PhantomData<(D1, D2, D3)>,
}

impl<D1, D2, D3, T> MaterialBuilder<T> for ThreeBoundedRectBuilder<D1, D2, D3> {
    type Finished = Rectangle<D1, D2, D3, T>;

    fn material(self, material: T) -> Self::Finished {
        Rectangle {
            d1_range: self.d1_range,
            d2_range: self.d2_range,
            d3: self.d3,
            material,
            tag: PhantomData,
        }
    }
}
