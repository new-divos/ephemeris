pub mod angle;
pub mod consts;
pub mod linalg;
pub mod error;

use std::convert::From;
use std::default::Default;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use num_traits::float::Float;

use crate::base::error::Error;


type Result<T> = std::result::Result<T, Error>;


pub trait Real<T = Self> where T: Float
{
    ///
    /// frac: Gives the fractional part of a number
    ///
    fn frac(self) -> Self;

    ///
    /// fmod: calculates x mod y
    ///
    fn fmod(self, rhs: Self) -> Self;
}

macro_rules! impl_real {
    ($t:ty) => (
        impl Real for $t {
            fn frac(self) -> Self {
                self - self.floor()
            }

            fn fmod(self, rhs: Self) -> Self {
                self - rhs * (self / rhs).floor()
            }
        }
    );
}

impl_real!(f64);
impl_real!(f32);


///
/// Pair: Calculates cos(alpha+beta) and sin(alpha+beta) using addition
/// theorems
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PertPair(f64, f64);

impl From<f64> for PertPair {
    #[inline]
    fn from(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Self(s, c)
    }
}

impl Default for PertPair {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Neg for PertPair {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, self.1)
    }
}

impl Add for PertPair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.1 + self.1 * rhs.0,
            self.1 * rhs.1 - self.0 * rhs.0
        )
    }
}

impl AddAssign for PertPair {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Mul<i32> for PertPair {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        if rhs == 0 {
            Self::new()
        } else if rhs == 1 {
            self
        } else if rhs == 2 {
            Self(
                2.0 * self.0 * self.1,
                self.1 * self.1 - self.0 * self.0
            )
        } else if rhs < 0 {
            let pair = self.mul(-rhs);
            pair.neg()
        } else if rhs & 1 == 0 {
            let pair = self.mul(rhs >> 1);
            pair.mul(2)
        } else {
            let pair = self.mul(rhs - 1);
            pair.add(self)
        }
    }
}

impl Mul<PertPair> for i32 {
    type Output = PertPair;

    #[inline]
    fn mul(self, rhs: PertPair) -> PertPair {
        rhs.mul(self)
    }
}

impl MulAssign<i32> for PertPair {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.mul(rhs);
    }
}

impl PertPair {
    #[inline]
    pub fn new() -> PertPair {
        PertPair(0.0, 1.0)
    }

    #[inline]
    pub fn sin(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn cos(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn sin_cos(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}
