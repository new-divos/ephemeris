use std::f64::consts::{FRAC_PI_2, PI};
use std::cmp;
use std::convert;
use std::default;
use std::fmt;
use std::iter;
use std::marker::PhantomData;
use std::ops;
use std::ops::{Mul, Div};

use crate::base::{Real, Result};
use crate::base::consts::PI2;
use crate::base::error::Error;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vec3D<T: Copy>([f64; 3], #[serde(skip)] PhantomData<T>);

impl<T: Copy> cmp::PartialEq for Vec3D<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}

impl<T: Copy> convert::Into<(f64, f64, f64)> for Vec3D<T> {
    #[inline]
    fn into(self) -> (f64, f64, f64) {
        (self.0[0], self.0[1], self.0[2])
    }
}

impl<T: Copy> default::Default for Vec3D<T> {
    #[inline]
    fn default() -> Self {
        Self::zeros()
    }
}

impl<'a, T: Copy> iter::IntoIterator for &'a Vec3D<T> {
    type Item = f64;
    type IntoIter = Vec3DIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Copy> ops::Index<usize> for Vec3D<T> {
    type Output = f64;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T: Copy> Vec3D<T> {
    #[inline]
    pub fn zeros() -> Self {
        Vec3D::<T>([0.0; 3], PhantomData::<T>)
    }

    #[inline]
    pub fn filled_by(value: f64) -> Self {
        Vec3D::<T>([value; 3], PhantomData::<T>)
    }

    pub fn get(&self, idx: usize) -> Option<f64> {
        match idx {
            0..=2 => Some(self.0[idx]),
            _ => None
        }
    }

    #[inline]
    fn iter(&self) -> Vec3DIterator<T> {
        Vec3DIterator::<T> {
            vector: self,
            count: 0
        }
    }
}

impl<T: Copy> Vec3D<T> where Vec3D<T>: ops::Div<f64> {
    #[inline]
    pub fn try_div(self, rhs: f64) -> Result<<Self as ops::Div<f64>>::Output> {
        if rhs != 0.0 {
            Ok(self.div(rhs))
        } else {
            Err(Error::ZeroDivisionError)
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vec3DIterator<'a, T: Copy> {
    vector: &'a Vec3D<T>,
    count: usize
}

impl<'a, T: Copy> iter::Iterator for Vec3DIterator<'a, T> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            let idx = self.count;
            self.count += 1;

            Some(self.vector.0[idx])
        } else {
            None
        }
    }
}

macro_rules! x {
    ($self:ident) => {
        $self.0[0]
    };
}

macro_rules! y {
    ($self:ident) => {
        $self.0[1]
    };
}

macro_rules! z {
    ($self:ident) => {
        $self.0[2]
    };
}


#[derive(Copy, Clone)]
pub struct Cartesian;

impl convert::Into<Vec3D<Cylindrical>> for Vec3D<Cartesian> {
    fn into(self) -> Vec3D<Cylindrical> {
        Vec3D::<Cylindrical>::new(
            y!(self).hypot(x!(self)),
            self.azimuth(),
            z!(self)
        )
    }
}

impl convert::Into<Vec3D<Spherical>> for Vec3D<Cartesian> {
    fn into(self) -> Vec3D<Spherical> {
        let rho_sq = x!(self) * x!(self) + y!(self) * y!(self);
        let r = (rho_sq + z!(self) * z!(self)).sqrt();

        let rho = rho_sq.sqrt();
        let theta = if rho == 0.0 && z!(self) == 0.0 {
            0.0
        } else {
            z!(self).atan2(rho)
        };

        Vec3D::<Spherical>::new(
            r,
            self.azimuth(),
            theta
        )
    }
}

impl fmt::Debug for Vec3D<Cartesian> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3D<Cartesian>")
            .field("x", &x!(self))
            .field("y", &y!(self))
            .field("z", &z!(self))
            .finish()
    }
}

impl fmt::Display for Vec3D<Cartesian> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x={}, y={}, z={}]", x!(self), y!(self), z!(self))
    }
}

impl ops::IndexMut<usize> for Vec3D<Cartesian> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl ops::Neg for Vec3D<Cartesian> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-x!(self), -y!(self), -z!(self))
    }
}

impl ops::Add for Vec3D<Cartesian> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::new(
            x!(self) + x!(other),
            y!(self) + y!(other),
            z!(self) + z!(other)
        )
    }
}

impl ops::AddAssign for Vec3D<Cartesian> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..3 {
            self.0[i] += other.0[i];
        }
    }
}

impl ops::Sub for Vec3D<Cartesian> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            x!(self) - x!(other),
            y!(self) - y!(other),
            z!(self) - z!(other)
        )
    }
}

impl ops::SubAssign for Vec3D<Cartesian> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..3 {
            self.0[i] -= other.0[i];
        }
    }
}

impl ops::Mul<f64> for Vec3D<Cartesian> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D::<Cartesian>::new(
            x!(self) * rhs,
            y!(self) * rhs,
            z!(self) * rhs
        )
    }
}

impl ops::Mul<Vec3D<Cartesian>> for f64 {
    type Output = Vec3D<Cartesian>;

    #[inline]
    fn mul(self, other: Vec3D<Cartesian>) -> Self::Output {
        other.mul(self)
    }
}

impl ops::MulAssign<f64> for Vec3D<Cartesian> {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.0[i] *= rhs;
        }
    }
}

impl ops::Div<f64> for Vec3D<Cartesian> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            x!(self) / rhs,
            y!(self) / rhs,
            z!(self) / rhs
        )
    }
}

impl ops::DivAssign<f64> for Vec3D<Cartesian> {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.0[i] /= rhs;
        }
    }
}

impl Vec3D<Cartesian> {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z], PhantomData::<Cartesian>)
    }

    #[inline]
    pub fn unit_x() -> Self {
        Self([1.0, 0.0, 0.0], PhantomData::<Cartesian>)
    }

    #[inline]
    pub fn unit_y() -> Self {
        Self([0.0, 1.0, 0.0], PhantomData::<Cartesian>)
    }

    #[inline]
    pub fn unit_z() -> Self {
        Self([0.0, 0.0, 1.0], PhantomData::<Cartesian>)
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        (x!(self) * x!(self) + y!(self) * y!(self) + z!(self) * z!(self)).sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Vec3D<Cartesian>) -> f64 {
        x!(self) * x!(other) + y!(self) * y!(other) + z!(self) * z!(other)
    }

    #[inline]
    pub fn cross(&self, other: &Vec3D<Cartesian>) -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>::new(
            y!(self) * z!(other) - z!(self) * y!(other), // y1 * z2 - z1 * y2
            z!(self) * x!(other) - x!(self) * z!(other), // z1 * x2 - x1 * z2
            x!(self) * y!(other) - y!(self) * x!(other)  // x1 * y2 - y1 * x2
        )
    }

    #[inline]
    pub fn x(&self) -> f64 { x!(self) }

    #[inline]
    pub fn y(&self) -> f64 { y!(self) }

    #[inline]
    pub fn z(&self) -> f64 { z!(self) }

    #[inline]
    fn azimuth(&self) -> f64 {
        if x!(self) == 0.0 && y!(self) == 0.0 {
            0.0
        } else {
            y!(self).atan2(x!(self))
        }
    }
}


#[derive(Copy, Clone)]
pub struct Cylindrical;

impl convert::Into<Vec3D<Cartesian>> for Vec3D<Cylindrical> {
    fn into(self) -> Vec3D<Cartesian> {
        let (sin_phi, cos_phi) = self.0[1].sin_cos();
        Vec3D::<Cartesian>::new(
            self.0[0] * cos_phi,
            self.0[0] * sin_phi,
            self.0[2]
        )
    }
}

impl convert::Into<Vec3D<Spherical>> for Vec3D<Cylindrical> {
    fn into(self) -> Vec3D<Spherical> {
        let theta = if self.0[0] == 0.0 && self.0[2] == 0.0 {
            0.0
        } else {
            self.0[2].atan2(self.0[0])
        };

        Vec3D::<Spherical>::new(
            self.0[0].hypot(self.0[2]),
            self.0[1],
            theta
        )
    }
}

impl fmt::Debug for Vec3D<Cylindrical> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3D<Cylindrical>")
            .field("radius", &self.0[0])
            .field("azimuth", &self.0[1])
            .field("altitude", &self.0[2])
            .finish()
    }
}

impl fmt::Display for Vec3D<Cylindrical> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[radius={}, azimuth={}, altitude={}]", self.0[0], self.0[1], self.0[2])
    }
}

impl ops::Neg for Vec3D<Cylindrical> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(self.0[0], self.0[1] + PI, -self.0[2])
    }
}

impl ops::Mul<f64> for Vec3D<Cylindrical> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.0[0] * rhs.abs(),
            if rhs < 0.0 { self.0[1] + PI } else { self.0[1] },
            self.0[2] * rhs
        )
    }
}

impl ops::Mul<Vec3D<Cylindrical>> for f64 {
    type Output = Vec3D<Cylindrical>;

    #[inline]
    fn mul(self, rhs: Vec3D<Cylindrical>) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Vec3D<Cylindrical> {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs.abs();
        if rhs < 0.0 {
            self.0[1] = self.0[1] + PI;
        }
        self.0[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3D<Cylindrical> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.0[0] / rhs.abs(),
            if rhs < 0.0 { self.0[1] + PI } else { self.0[1] },
            self.0[2] / rhs
        )
    }
}

impl ops::DivAssign<f64> for Vec3D<Cylindrical> {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs.abs();
        if rhs < 0.0 {
            self.0[1] = (self.0[1] + PI).fmod(PI2);
        }
        self.0[2] /= rhs;
    }
}


impl Vec3D<Cylindrical> {
    #[inline]
    pub fn new(radius: f64, azimuth: f64, altitude: f64) -> Self {
        Self(
            [
                radius.abs(),
                if radius < 0.0 {
                    (azimuth + PI).fmod(PI2)
                } else {
                    azimuth.fmod(PI2)
                },
                altitude
            ],
            PhantomData::<Cylindrical>
        )
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.0[2].hypot(self.0[0])
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn azimuth(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn altitude(&self) -> f64 {
        self.0[2]
    }
}


#[derive(Copy, Clone)]
pub enum Spherical {
    Radius,
    Azimuth,
    Colatitude
}

impl convert::Into<Vec3D<Cartesian>> for Vec3D<Spherical> {
    fn into(self) -> Vec3D<Cartesian> {
        let (r, phi, theta) = self.into();

        let (phi_sin, phi_cos) = phi.sin_cos();
        let (theta_sin, theta_cos) = theta.sin_cos();
        let rho = r * theta_cos;
        Vec3D::<Cartesian>::new(rho * phi_cos, rho * phi_sin, r * theta_sin)
    }
}

impl convert::Into<Vec3D<Cylindrical>> for Vec3D<Spherical> {
    fn into(self) -> Vec3D<Cylindrical> {
        let (r, phi, theta) = self.into();

        let (theta_sin, theta_cos) = theta.sin_cos();
        Vec3D::<Cylindrical>::new(r * theta_cos, phi, r * theta_sin)
    }
}

impl fmt::Debug for Vec3D<Spherical> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3D<Spherical>")
            .field("radius", &self.0[0])
            .field("azimuth", &self.0[1])
            .field("colatitude", &self.0[2])
            .finish()
    }
}

impl fmt::Display for Vec3D<Spherical> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[radius={}, azimuth={}, colatitude={}]", self.0[0], self.0[1], self.0[2])
    }
}

impl ops::Index<Spherical> for Vec3D<Spherical> {
    type Output = f64;

    fn index(&self, idx: Spherical) -> &Self::Output {
        match idx {
            Spherical::Radius => &self.0[0],
            Spherical::Azimuth => &self.0[1],
            Spherical::Colatitude => &self.0[2]
        }
    }
}

impl ops::Neg for Vec3D<Spherical> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (r, phi, theta) = self.into();
        Self::new(r, phi + PI, -theta)
    }
}

impl ops::Mul<f64> for Vec3D<Spherical> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let (r, phi, theta) = self.into();
        Self::new(r * rhs, phi, theta)
    }
}

impl ops::Mul<Vec3D<Spherical>> for f64 {
    type Output = Vec3D<Spherical>;

    #[inline]
    fn mul(self, rhs: Vec3D<Spherical>) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Vec3D<Spherical> {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs.abs();
        if rhs < 0.0 {
            self.0[1] = (self.0[1] + PI).fmod(PI2);
            self.0[2] = -self.0[2];
        }
    }
}

impl ops::Div<f64> for Vec3D<Spherical> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (r, phi, theta) = self.into();
        Self::new(r / rhs, phi, theta)
    }
}

impl ops::DivAssign<f64> for Vec3D<Spherical> {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs.abs();
        if rhs < 0.0 {
            self.0[1] = (self.0[1] + PI).fmod(PI2);
            self.0[2] = -self.0[2];
        }
    }
}

impl Vec3D<Spherical> {
    #[inline]
    pub fn radius(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn azimuth(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn colatitude(&self) -> f64 {
        self.0[2]
    }

    pub fn new(radius: f64, azimuth: f64, colatitude: f64) -> Vec3D<Spherical> {
        Vec3D::<Spherical>(
            [
                radius.abs(),
                if radius < 0.0 {
                    (azimuth + PI).fmod(PI2)
                } else {
                    azimuth.fmod(PI2)
                },
                Vec3D::<Spherical>::clamp(
                    if radius < 0.0 {
                        -colatitude
                    } else {
                        colatitude
                    }
                )
            ],
            PhantomData::<Spherical>
        )
    }

    pub fn unit(azimuth: f64, colatitude: f64) -> Vec3D<Spherical> {
        Vec3D::<Spherical>(
            [
                1.0,
                azimuth.fmod(PI2),
                Vec3D::<Spherical>::clamp(colatitude)
            ],
            PhantomData::<Spherical>
        )
    }

    pub fn norm(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    fn clamp(theta: f64) -> f64 {
        if theta < -FRAC_PI_2 {
            -FRAC_PI_2
        } else if theta > FRAC_PI_2 {
            FRAC_PI_2
        } else {
            theta
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mat3D([[f64; 3]; 3]);

impl default::Default for Mat3D {
    fn default() -> Self {
        Mat3D::zeros()
    }
}

pub type Mat3DRow = [f64; 3];

impl ops::Index<usize> for Mat3D {
    type Output = Mat3DRow;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Mat3D {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl ops::Neg for Mat3D {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self;

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = -result.0[i][j];
            }
        }

        result
    }
}

impl ops::Add for Mat3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = self;

        result += rhs;
        result
    }
}

impl ops::AddAssign for Mat3D {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            for j in 0..3 {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl ops::Sub for Mat3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = self;

        result -= rhs;
        result
    }
}

impl ops::SubAssign for Mat3D {
    fn sub_assign(&mut self, rhs: Mat3D) {
        for i in 0..3 {
            for j in 0..3 {
                self.0[i][j] -= rhs.0[i][j];
            }
        }
    }
}

impl ops::Mul<f64> for Mat3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = self;

        result *= rhs;
        result
    }
}

impl ops::Mul<Mat3D> for f64 {
    type Output = Mat3D;

    #[inline]
    fn mul(self, rhs: Mat3D) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Mat3D {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            for j in 0..3 {
                self.0[i][j] *= rhs;
            }
        }
    }
}

impl ops::Mul<Vec3D<Cartesian>> for Mat3D {
    type Output = Vec3D<Cartesian>;

    fn mul(self, rhs: Vec3D<Cartesian>) -> Self::Output {
        let mut values = [0.0; 3];
        for i in 0..3 {
            for j in 0..3 {
                values[i] += self.0[i][j] * rhs.0[j];
            }
        }

        Vec3D::<Cartesian>(values, PhantomData::<Cartesian>)
    }
}

impl ops::Mul for Mat3D {
    type Output = Self;

    fn mul(self, rhs: Mat3D) -> Mat3D {
        let mut result = Mat3D::zeros();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.0[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        result
    }
}

impl ops::MulAssign for Mat3D {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl ops::Div<f64> for Mat3D {
    type Output = Mat3D;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self;

        result /= rhs;
        result
    }
}

impl ops::DivAssign<f64> for Mat3D {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            for j in 0..3 {
                self.0[i][j] /= rhs;
            }
        }
    }
}

impl ops::Div for Mat3D {
    type Output = Result<Mat3D>;

    fn div(self, rhs: Mat3D) -> Self::Output {
        let inverted = rhs.inv()?;
        Ok(inverted.mul(self))
    }
}

impl ops::Div<Mat3D> for f64 {
    type Output = Result<Mat3D>;

    fn div(self, rhs: Mat3D) -> Self::Output {
        let inverted = rhs.inv()?;
        Ok(inverted.mul(self))
    }
}

impl ops::Div<Mat3D> for Vec3D<Cartesian> {
    type Output = Result<Vec3D<Cartesian>>;

    fn div(self, rhs: Mat3D) -> Self::Output {
        let inverted = rhs.inv()?;
        Ok(inverted.mul(self))
    }
}

impl Mat3D {
    #[inline]
    pub fn zeros() -> Mat3D {
        Mat3D([[0.0; 3]; 3])
    }

    #[inline]
    pub fn filled_by(value: f64) -> Mat3D {
        Mat3D([[value; 3]; 3])
    }

    #[inline]
    pub fn identity() -> Mat3D {
        Mat3D(
            [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        )
    }

    #[inline]
    pub fn with_rows(r1: &Vec3D<Cartesian>,
                     r2: &Vec3D<Cartesian>,
                     r3: &Vec3D<Cartesian>) -> Mat3D {
        Mat3D(
            [
                [r1.0[0], r1.0[1], r1.0[2]],
                [r2.0[0], r2.0[1], r2.0[2]],
                [r3.0[0], r3.0[1], r3.0[2]]
            ]
        )
    }

    #[inline]
    pub fn with_columns(c1: &Vec3D<Cartesian>,
                        c2: &Vec3D<Cartesian>,
                        c3: &Vec3D<Cartesian>) -> Mat3D {
        Mat3D(
            [
                [c1.0[0], c2.0[0], c3.0[0]],
                [c1.0[1], c2.0[1], c3.0[1]],
                [c1.0[2], c2.0[2], c3.0[2]]
            ]
        )
    }

    #[inline]
    pub fn diag(vector: &Vec3D<Cartesian>) -> Mat3D {
        Mat3D(
            [
                [vector.0[0],         0.0,        0.0],
                [        0.0, vector.0[1],        0.0],
                [        0.0,         0.0, vector.0[2]]
            ]
        )
    }

    pub fn r_x(angle: f64) -> Mat3D {
        let (angle_sin, angle_cos) = angle.sin_cos();

        Mat3D(
            [
                [1.0,        0.0,       0.0],
                [0.0,  angle_cos, angle_sin],
                [0.0, -angle_sin, angle_cos]
            ]
        )
    }

    pub fn r_y(angle: f64) -> Mat3D {
        let (angle_sin, angle_cos) = angle.sin_cos();

        Mat3D(
            [
                [angle_cos, 0.0, -angle_sin],
                [      0.0, 1.0,        0.0],
                [angle_sin, 0.0,  angle_cos]
            ]
        )
    }

    pub fn r_z(angle: f64) -> Mat3D {
        let (angle_sin, angle_cos) = angle.sin_cos();

        Mat3D(
            [
                [ angle_cos, angle_sin, 0.0],
                [-angle_sin, angle_cos, 0.0],
                [       0.0,       0.0, 1.0]
            ]
        )
    }

    #[inline]
    pub fn row(&self, index: usize) -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>::new(
            self.0[index][0],
            self.0[index][1],
            self.0[index][2]
        )
    }

    #[inline]
    pub fn column(&self, index: usize) -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>::new(
            self.0[0][index],
            self.0[1][index],
            self.0[2][index]
        )
    }

    #[inline]
    pub fn t(&self) -> Mat3D {
        Mat3D(
            [
                [self.0[0][0], self.0[1][0], self.0[2][0]],
                [self.0[0][1], self.0[1][1], self.0[2][1]],
                [self.0[0][2], self.0[1][2], self.0[2][2]]
            ]
        )
    }

    #[inline]
    pub fn tr(&self) -> f64 {
        self.0[0][0] + self.0[1][1] + self.0[2][2]
    }

    #[inline]
    pub fn det(&self) -> f64 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] -
                self.0[1][2] * self.0[2][1]) -
            self.0[0][1] * (self.0[1][0] * self.0[2][2] -
                self.0[1][2] * self.0[2][0]) +
            self.0[0][2] * (self.0[1][0] * self.0[2][1] -
                self.0[1][1] * self.0[2][0])
    }

    pub fn inv(&self) -> Result<Mat3D> {
        let det = self.det();
        if det == 0.0 {
            return Err(Error::SingularMatrixError);
        }

        let d2 = |r1: usize, c1: usize, r2: usize, c2: usize| {
            self.0[r1][c1] * self.0[r2][c2] - self.0[r1][c2] * self.0[r2][c1]
        };

        Ok(
            Mat3D(
                [
                    [
                         d2(1, 1, 2, 2) / det,
                        -d2(0, 1, 2, 2) / det,
                         d2(0, 1, 1, 2) / det
                    ],
                    [
                        -d2(1, 0, 2, 2) / det,
                         d2(0, 0, 2, 2) / det,
                        -d2(0, 0, 1, 2) / det
                    ],
                    [
                         d2(1, 0, 2, 1) / det,
                        -d2(0, 0, 2, 1) / det,
                         d2(0, 0, 1, 1) / det
                    ],
                ]
            )
        )
    }

    #[inline]
    pub fn try_div(&self, rhs: f64) -> Result<Mat3D> {
        if rhs == 0.0 {
            Ok(self.div(rhs))
        } else {
            Err(Error::ZeroDivisionError)
        }
    }

    #[inline]
    pub fn iter(&self) -> Mat3DIterator {
        Mat3DIterator {
            matrix: self,
            count: 0
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mat3DIterator<'a> {
    matrix: &'a Mat3D,
    count: usize
}

impl<'a> iter::Iterator for Mat3DIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 9 {
            let row = self.count / 3;
            let col = self.count % 3;

            self.count += 1;

            Some(self.matrix.0[row][col])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::tests::{EPS, ITERATIONS};


    fn new_random_mat3d<R: Rng + ?Sized>(rng: &mut R) -> Mat3D {
        let mut result = Mat3D::zeros();

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = rng.gen::<f64>();
            }
        }

        result
    }

    #[test]
    fn mat3d_add_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let mut m1 = new_random_mat3d(&mut rng);
            let m2 = new_random_mat3d(&mut rng);

            let m3 = m1 + m2;
            for i in 0..2 {
                for j in 0..2 {
                    assert_eq!(m3.0[i][j], m1.0[i][j] + m2.0[i][j]);
                }
            }

            m1 += m2;
            for i in 0..2 {
                for j in 0..2 {
                    assert_eq!(m1.0[i][j], m3.0[i][j]);
                }
            }
        }
    }

    #[test]
    fn mat3d_sub_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let mut m1 = new_random_mat3d(&mut rng);
            let m2 = new_random_mat3d(&mut rng);

            let m3 = m1 - m2;
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(m3.0[i][j], m1.0[i][j] - m2.0[i][j]);
                }
            }

            m1 -= m2;
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(m1.0[i][j], m3.0[i][j]);
                }
            }
        }
    }

    #[test]
    fn mat3d_num_mul_test() {
        let a = Mat3D(
            [
                [5.0, 8.0, -4.0],
                [6.0, 9.0, -5.0],
                [4.0, 7.0, -3.0]
            ]
        );

        let b = 3.0 * a;
        for (v1, v2) in a.iter().zip(b.iter()) {
            assert_relative_eq!(3.0 * v1, v2);
        }

        let b = a * 3.0;
        for (v1, v2) in a.iter().zip(b.iter()) {
            assert_relative_eq!(v1 * 3.0, v2);
        }

        let c = a / 3.0;
        for (v1, v2) in a.iter().zip(c.iter()) {
            assert_relative_eq!(v1 / 3.0, v2);
        }

        let mut rng = rand::thread_rng();
        for _ in 0..ITERATIONS {
            let a = new_random_mat3d(&mut rng);
            let k: f64 = 200.0 * rng.gen::<f64>() - 100.0;

            let b = k * a;
            for (v1, v2) in a.iter().zip(b.iter()) {
                assert_relative_eq!(k * v1, v2);
            }

            let b = a * k;
            for (v1, v2) in a.iter().zip(b.iter()) {
                assert_relative_eq!(v1 * k, v2);
            }

            if k == 0.0 {
                continue;
            }

            let b = a / k;
            for (v1, v2) in a.iter().zip(b.iter()) {
                assert_relative_eq!(v1 / k, v2);
            }
        }
    }

    #[test]
    fn mat3d_mul_test() {
        let a = Mat3D(
            [
                [5.0, 8.0, -4.0],
                [6.0, 9.0, -5.0],
                [4.0, 7.0, -3.0]
            ]
        );
        let b = Mat3D(
            [
                [3.0,  2.0, 5.0],
                [4.0, -1.0, 3.0],
                [9.0,  6.0, 5.0]
            ]
        );
        let c = Mat3D(
            [
                [11.0, -22.0, 29.0],
                [ 9.0, -27.0, 32.0],
                [13.0, -17.0, 26.0]
            ]
        );

        assert_eq!(a * b, c);

        let mut d = a;
        d *= b;

        assert_eq!(d, c);

        let a = Mat3D(
            [
                [ 1.0,  3.0,  4.0],
                [-1.0,  7.0,  9.0],
                [ 4.0,  3.0,  8.0]
            ]
        );
        let b = Mat3D(
            [
                [9.0,  1.0,  4.0],
                [1.0, -9.0,  5.0],
                [1.0,  2.0,  9.0]
            ]
        );
        let c = Mat3D(
            [
                [16.0, -18.0,  55.0],
                [ 7.0, -46.0, 112.0],
                [47.0,  -7.0, 103.0]
            ]
        );

        assert_eq!(a * b, c);

        let mut d = a;
        d *= b;

        assert_eq!(d, c);
    }

    #[test]
    fn mat3d_transpose_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let a = new_random_mat3d(&mut rng);
            let b = a.t();

            for i in 0..2 {
                for j in 0..2 {
                    assert_eq!(b.0[i][j], a.0[j][i]);
                }
            }
        }
    }

    #[test]
    fn mat3d_det_test() {
        let a = Mat3D(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        );
        assert_eq!(a.det(), 0.0);

        let b = Mat3D(
            [
                [1.0,  4.0, 8.0],
                [8.0, -3.0, 4.0],
                [4.0,  8.0, 8.0]
            ]
        );
        assert_eq!(b.det(), 360.0);

        let c = Mat3D(
            [
                [1.0, 2.0,  3.0],
                [4.0, 5.0,  6.0],
                [7.0, 8.0, -9.0]
            ]
        );
        assert_eq!(c.det(), 54.0);
    }

    #[test]
    fn mat3d_inv_test() {
        let a = Mat3D(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        );

        let r = a.inv();
        assert!(r.is_err());

        let a = Mat3D(
            [
                [3.0, 2.0, 2.0],
                [1.0, 3.0, 1.0],
                [5.0, 3.0, 4.0]
            ]
        );
        let b = Mat3D(
            [
                [  9.0 / 5.0, -2.0 / 5.0,  -4.0 / 5.0],
                [  1.0 / 5.0,  2.0 / 5.0,  -1.0 / 5.0],
                [-12.0 / 5.0,  1.0 / 5.0,   7.0 / 5.0]
            ]
        );

        let inv_a = a.inv().unwrap();
        for (v1, v2) in inv_a.iter().zip(b.iter()) {
            assert_relative_eq!(v1, v2);
        }

        let inv_a = (1.0 / a).unwrap();
        for (v1, v2) in inv_a.iter().zip(b.iter()) {
            assert_relative_eq!(v1, v2);
        }

        let a = Mat3D(
            [
                [4.0, 8.0, 0.0],
                [8.0, 8.0, 8.0],
                [2.0, 0.0, 1.0]
            ]
        );
        let b = Mat3D(
            [
                [ 1.0 / 12.0, -1.0 / 12.0,  2.0 / 3.0],
                [ 1.0 / 12.0,  1.0 / 24.0, -1.0 / 3.0],
                [-1.0 /  6.0,  1.0 /  6.0, -1.0 / 3.0]
            ]
        );

        let inv_a = a.inv().unwrap();
        for (v1, v2) in inv_a.iter().zip(b.iter()) {
            assert_relative_eq!(v1, v2);
        }

        let vk = vec![-2.0, -1.0, -0.5, 0.5, 1.0, 2.0];
        for k in vk {
            let c = (k / a).unwrap();
            for (v1, v2) in b.iter().map(|x| x * k).zip(c.iter()) {
                assert_relative_eq!(v1, v2);
            }
        }

        let e = Mat3D::identity();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let a = new_random_mat3d(&mut rng);
            let r = a.inv();
            match r {
                Ok(inv_a) => {
                    let e_test = a * inv_a;
                    for (v1, v2) in e_test.iter().zip(e.iter()) {
                        assert_relative_eq!(v1, v2, epsilon = self::EPS);
                    }

                    let e_test = inv_a * a;
                    for (v1, v2) in e_test.iter().zip(e.iter()) {
                        assert_relative_eq!(v1, v2, epsilon = self::EPS);
                    }
                },
                Err(_) => continue
            }
        }
    }

    #[test]
    fn mat3d_iter_test() {
        let a = Mat3D(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        );

        for (i, m) in a.iter().enumerate() {
            assert_eq!((i + 1) as f64, m);
        }
    }

    #[test]
    fn mat3d_row_column_test() {
        let a = Mat3D(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        );

        for i in 0..3 {
            let row = a.row(i);
            for j in 0..3 {
                assert_eq!(row[j], a.0[i][j]);
            }
        }

        for j in 0..3 {
            let column = a.column(j);
            for i in 0..3 {
                assert_eq!(column[i], a.0[i][j]);
            }
        }
    }

    #[test]
    fn mat3d_index_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let a = new_random_mat3d(&mut rng);
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(a[i][j], a.0[i][j]);
                }
            }

            let mut b = Mat3D::zeros();
            for i in 0..3 {
                for j in 0..3 {
                    b[i][j] = a[i][j];
                }
            }

            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(b.0[i][j], a.0[i][j]);
                }
            }
        }
    }
}
