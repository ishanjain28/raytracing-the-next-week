use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, RangeInclusive, Sub,
        SubAssign,
    },
};

use rand::Rng;

use crate::{Asf64, Dimension, X, Y, Z};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    #[inline]
    pub fn new(a: impl Asf64, b: impl Asf64, c: impl Asf64) -> Vec3 {
        Vec3([a.as_(), b.as_(), c.as_()])
    }

    pub fn splat(xyz: impl Asf64) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self(rng.gen())
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
        self.0[D::INDEX]
    }

    pub fn set<D: Dimension>(mut self, value: f64) -> Self {
        self.0[D::INDEX] = value;
        self
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn sq_len(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    #[inline]
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    #[inline]
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3([
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        ])
    }

    #[inline]
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn min(self, other: Self) -> Vec3 {
        Self([
            self.x().min(other.x()),
            self.y().min(other.y()),
            self.z().min(other.z()),
        ])
    }

    pub fn max(self, other: Self) -> Vec3 {
        Self([
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
        ])
    }

    pub fn min_element(self) -> f64 {
        self.x().min(self.y()).min(self.z())
    }

    pub fn max_element(self) -> f64 {
        self.x().max(self.y()).max(self.z())
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        Vec3::new(self.x().sqrt(), self.y().sqrt(), self.z().sqrt())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3([self.x() + o.x(), self.y() + o.y(), self.z() + o.z()])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Vec3) {
        self.0[0] += o.0[0];
        self.0[1] += o.0[1];
        self.0[2] += o.0[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3([self.x() - o.x(), self.y() - o.y(), self.z() - o.z()])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Vec3) {
        self.0[0] -= o.0[0];
        self.0[1] -= o.0[1];
        self.0[2] -= o.0[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3([-self.x(), -self.y(), -self.z()])
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, o: Vec3) {
        self.0[0] *= o.0[0];
        self.0[1] *= o.0[1];
        self.0[2] *= o.0[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, o: f64) {
        self.0[0] *= o;
        self.0[1] *= o;
        self.0[2] *= o;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: f64) -> Vec3 {
        Vec3([self.x() * o, self.y() * o, self.z() * o])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: Vec3) -> Vec3 {
        Vec3([self.x() * o.x(), self.y() * o.y(), self.z() * o.z()])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3([self.x() / o.x(), self.y() / o.y(), self.z() / o.z()])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f64) -> Vec3 {
        let o = 1.0 / o;
        Vec3([self.x() * o, self.y() * o, self.z() * o])
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, o: f64) {
        let o = 1.0 / o;
        self.0[0] *= o;
        self.0[1] *= o;
        self.0[2] *= o;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, q: usize) -> &f64 {
        &self.0[q]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, q: usize) -> &mut f64 {
        &mut self.0[q]
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
