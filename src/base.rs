pub mod consts;
pub mod linalg;

use std::convert::From;
use std::default::Default;
use std::ops::{Add, AddAssign, Neg};

///
/// Frac: Gives the fractional part of a number
///
pub trait Fractional {
    fn fractional(self) -> Self;
}

macro_rules! impl_fractional {
    ($t:ty) => (
        impl Fractional for $t {
            fn fractional(self) -> Self {
                self - self.floor()
            }
        }
    );
}

impl_fractional!(f32);
impl_fractional!(f64);

///
/// FMod: calculates x mod y
///
pub trait Modulo {
    fn modulo(self, rhs: Self) -> Self;
}

macro_rules! impl_modulo {
    ($t:ty) => (
        impl Modulo for $t {
            fn modulo(self, rhs: Self) -> Self {
                self - rhs * (self / rhs).floor()
            }
        }
    );
}

impl_modulo!(f32);
impl_modulo!(f64);

///
/// Pair: Calculates cos(alpha+beta) and sin(alpha+beta) using addition
/// theorems
///
#[derive(Debug, Copy, Clone)]
pub struct PertPair {
    c: f64,
    s: f64
}

impl From<f64> for PertPair {
    fn from(angle: f64) -> Self {
        let v = angle.sin_cos();
        Self { c: v.1, s: v.0 }
    }
}

impl Default for PertPair {
    fn default() -> Self {
        Self { c: 1.0, s: 0.0 }
    }
}

impl Neg for PertPair {
    type Output = Self;

    fn neg(self) -> Self {
        PertPair { c: self.c, s: -self.s }
    }
}

impl Add for PertPair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            c: self.c * rhs.c - self.s * rhs.s,
            s: self.s * rhs.c + self.c * rhs.s
        }
    }
}

impl AddAssign for PertPair {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl PertPair {
    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn s(&self) -> f64 {
        self.s
    }
}
