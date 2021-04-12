use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeInclusive, Sub, SubAssign},
};

use rand::Rng;

use crate::{Asf64, Dimension, X, Y, Z};

use packed_simd::{f64x4, shuffle};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3(f64x4);

impl Vec3 {
    #[inline]
    pub fn new(a: impl Asf64, b: impl Asf64, c: impl Asf64) -> Vec3 {
        Self(f64x4::new(a.as_(), b.as_(), c.as_(), 0.0))
    }

    pub fn splat(xyz: impl Asf64) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self(f64x4::from_slice_aligned(&rng.gen::<[f64; 4]>()))
    }

    pub fn random_in_range<R: Rng + ?Sized>(rng: &mut R, range: RangeInclusive<f64>) -> Self {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.get::<X>()
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.get::<Y>()
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.get::<Z>()
    }

    pub fn get<D: Dimension>(&self) -> f64 {
        unsafe { self.0.extract_unchecked(D::INDEX) }
    }

    pub fn set<D: Dimension>(self, value: f64) -> Self {
        Self(unsafe { self.0.replace_unchecked(D::INDEX, value) })
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn sq_len(&self) -> f64 {
        (self.0 * self.0).sum()
    }

    #[inline]
    pub fn dot(&self, v: &Vec3) -> f64 {
        (self.0 * v.0).sum()
    }

    #[inline]
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        // https://web.archive.org/web/20210412192227/https://geometrian.com/programming/tutorials/cross-product/index.php
        let tmp0: f64x4 = shuffle!(self.0, [1, 2, 0, 3]);
        let tmp1: f64x4 = shuffle!(v.0, [2, 0, 1, 3]);
        let tmp2: f64x4 = shuffle!(self.0, [2, 0, 1, 3]);
        let tmp3: f64x4 = shuffle!(v.0, [1, 2, 0, 3]);

        Vec3(tmp0 * tmp1 - tmp2 * tmp3)
    }

    #[inline]
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn min(self, other: Self) -> Vec3 {
        Self(self.0.min(other.0))
    }

    pub fn max(self, other: Self) -> Vec3 {
        Self(self.0.max(other.0))
    }

    pub fn min_element(self, other: f64) -> f64 {
        unsafe { self.0.replace_unchecked(3, other).min_element() }
    }

    pub fn max_element(self, other: f64) -> f64 {
        unsafe { self.0.replace_unchecked(3, other).max_element() }
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3(self.0 + o.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Vec3) {
        self.0 += o.0
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3(self.0 - o.0)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Vec3) {
        self.0 -= o.0;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, o: Vec3) {
        self.0 *= o.0
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, o: f64) {
        self.0 *= o
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: f64) -> Vec3 {
        Vec3(self.0 * o)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: Vec3) -> Vec3 {
        Vec3(self.0 * o.0)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3(self.0 / o.0)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f64) -> Vec3 {
        let o = 1.0 / o;

        Vec3(self.0 * o)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, o: f64) {
        let o = 1.0 / o;

        self.0 *= o
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.get::<X>(),
            self.get::<Y>(),
            self.get::<Z>()
        ))
    }
}

impl<A: Asf64, B: Asf64, C: Asf64> From<(A, B, C)> for Vec3 {
    fn from((x, y, z): (A, B, C)) -> Self {
        Self::new(x, y, z)
    }
}
