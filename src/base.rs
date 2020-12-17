use std::ops;
use std::ops::Add;

pub mod consts;
pub mod linalg;

///
/// Frac: Gives the fractional part of a number
///
pub trait Frac {
    fn frac(self) -> Self;
}

macro_rules! impl_frac {
    ($t:ty) => (
        impl Frac for $t {
            fn frac(self) -> Self {
                self - self.floor()
            }
        }
    );
}

impl_frac!(f32);
impl_frac!(f64);

///
/// FMod: calculates x mod y
///
pub trait FMod {
    fn fmod(self, rhs: Self) -> Self;
}

macro_rules! impl_fmod {
    ($t:ty) => (
        impl FMod for $t {
            fn fmod(self, rhs: Self) -> Self {
                self - rhs * (self / rhs).floor()
            }
        }
    );
}

impl_fmod!(f32);
impl_fmod!(f64);

///
/// Pair: Calculates cos(alpha+beta) and sin(alpha+beta) using addition
/// theorems
///
#[derive(Debug, Copy, Clone)]
pub struct Pair {
    pub c: f64,
    pub s: f64
}

impl ops::Neg for Pair {
    type Output = Self;

    fn neg(self) -> Self {
        Pair { c: self.c, s: -self.s }
    }
}

impl ops::Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            c: self.c * rhs.c - self.s * rhs.s,
            s: self.s * rhs.c + self.c * rhs.s
        }
    }
}

impl ops::AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Pair {
    fn from_zero() -> Pair {
        Self { c: 1.0, s: 0.0 }
    }

    fn from_angle(angle: f64) -> Pair {
        let v = angle.sin_cos();
        Self { c: v.1, s: v.0 }
    }
}
