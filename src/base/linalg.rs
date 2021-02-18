use std::f64::consts::{FRAC_PI_2, PI};
use std::convert;
use std::default;
use std::iter;
use std::marker::PhantomData;
use std::ops;
use std::ops::{Add, Mul, Sub, Div};

use crate::base::{Real, Result};
use crate::base::consts::PI2;
use crate::base::error::Error;


pub trait Vec3DNorm {
    fn norm(&self) -> f64;
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3D<T: Copy>([f64; 3], #[serde(skip)] PhantomData<T>);

impl<T: Copy> convert::Into<(f64, f64, f64)> for Vec3D<T> {
    fn into(self) -> (f64, f64, f64) {
        let Vec3D::<T>(ref array, _) = self;
        (array[0], array[1], array[2])
    }
}

impl<T: Copy> ops::Index<usize> for Vec3D<T> {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T: Copy> Vec3D<T> {
    pub fn zero() -> Self {
        Vec3D::<T>([0.0; 3], PhantomData::<T>{})
    }

    pub fn filled_by(value: f64) -> Self {
        Vec3D::<T>([value; 3], PhantomData::<T>{})
    }

    pub fn get(&self, idx: usize) -> Option<f64> {
        match idx {
            0..=2 => Some(self.0[idx]),
            _ => None
        }
    }

    pub fn iter(&self) -> Vec3DIterator<T> {
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


#[derive(Copy, Clone)]
pub enum Cartesian {
    X, Y, Z
}

impl convert::Into<Vec3D<Cylindrical>> for Vec3D<Cartesian> {
    fn into(self) -> Vec3D<Cylindrical> {
        let (x, y, z) = self.into();

        let phi = if x == 0.0 && y == 0.0 {
            0.0
        } else {
            y.atan2(x)
        };

        Vec3D::<Cylindrical>::new(x.hypot(y), phi, z)
    }
}

impl convert::Into<Vec3D<Spherical>> for Vec3D<Cartesian> {
    fn into(self) -> Vec3D<Spherical> {
        let (x, y, z) = self.into();

        let rho_sq = x * x + y * y;
        let r = (rho_sq + z * z).sqrt();

        let phi = if x == 0.0 && y == 0.0 {
            0.0
        } else {
            y.atan2(x)
        };

        let rho = rho_sq.sqrt();
        let theta = if rho == 0.0 && z == 0.0 {
            0.0
        } else {
            z.atan2(rho)
        };

        Vec3D::<Spherical>::new(r, phi, theta)
    }
}

impl ops::IndexMut<usize> for Vec3D<Cartesian> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl ops::Index<Cartesian> for Vec3D<Cartesian> {
    type Output = f64;

    fn index(&self, idx: Cartesian) -> &Self::Output {
        match idx {
            Cartesian::X => &self.0[0],
            Cartesian::Y => &self.0[1],
            Cartesian::Z => &self.0[2]
        }
    }
}

impl ops::IndexMut<Cartesian> for Vec3D<Cartesian> {
    fn index_mut(&mut self, idx: Cartesian) -> &mut Self::Output {
        match idx {
            Cartesian::X => &mut self.0[0],
            Cartesian::Y => &mut self.0[1],
            Cartesian::Z => &mut self.0[2]
        }
    }
}

impl ops::Neg for Vec3D<Cartesian> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (x, y, z) = self.into();
        Self::new(-x, -y, -z)
    }
}

impl ops::Add for Vec3D<Cartesian> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2]
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

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2]
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

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D::<Cartesian>::new(
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs
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

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs
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

impl Vec3DNorm for Vec3D<Cartesian> {
    fn norm(&self) -> f64 {
        let (x, y, z) = (*self).into();
        (x * x + y * y + z * z).sqrt()
    }
}

impl Vec3D<Cartesian> {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>([x, y, z], PhantomData::<Cartesian>{})
    }

    pub fn unit_x() -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>([1.0, 0.0, 0.0], PhantomData::<Cartesian>{})
    }

    pub fn unit_y() -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>([0.0, 1.0, 0.0], PhantomData::<Cartesian>{})
    }

    pub fn unit_z() -> Vec3D<Cartesian> {
        Vec3D::<Cartesian>([0.0, 0.0, 1.0], PhantomData::<Cartesian>{})
    }
}


#[derive(Copy, Clone)]
pub enum Cylindrical {
    Radius,
    Azimuth,
    Altitude
}

impl convert::Into<Vec3D<Cartesian>> for Vec3D<Cylindrical> {
    fn into(self) -> Vec3D<Cartesian> {
        let (rho, phi, z) = self.into();

        let (sin_phi, cos_phi) = phi.sin_cos();
        Vec3D::<Cartesian>::new(rho * cos_phi, rho * sin_phi, z)
    }
}

impl convert::Into<Vec3D<Spherical>> for Vec3D<Cylindrical> {
    fn into(self) -> Vec3D<Spherical> {
        let (rho, phi, z) = self.into();

        let theta = if rho == 0.0 && z == 0.0 {
            0.0
        } else {
            z.atan2(rho)
        };

        Vec3D::<Spherical>::new(rho.hypot(z), phi, theta)
    }
}

impl ops::Index<Cylindrical> for Vec3D<Cylindrical> {
    type Output = f64;

    fn index(&self, idx: Cylindrical) -> &Self::Output {
        match idx {
            Cylindrical::Radius => &self.0[0],
            Cylindrical::Azimuth => &self.0[1],
            Cylindrical::Altitude => &self.0[2]
        }
    }
}

impl ops::Neg for Vec3D<Cylindrical> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (rho, phi, z) = self.into();
        Self::new(rho, phi + PI, -z)
    }
}

impl ops::Mul<f64> for Vec3D<Cylindrical> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let (rho, phi, z) = self.into();
        Self::new(
            rho * rhs.abs(),
            if rhs < 0.0 {
                phi + PI
            } else {
                phi
            },
            z * rhs
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
            self.0[1] = (self.0[1] + PI).fmod(PI2);
        }
        self.0[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3D<Cylindrical> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (rho, phi, z) = self.into();
        Self::new(
            rho / rhs.abs(),
            if rhs < 0.0 {
                phi + PI
            } else {
                phi
            },
            z / rhs
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

impl Vec3DNorm for Vec3D<Cylindrical> {
    fn norm(&self) -> f64 {
        let (rho, _, z) = (*self).into();
        rho.hypot(z)
    }
}

impl Vec3D<Cylindrical> {
    pub fn new(rho: f64, phi: f64, z: f64) -> Vec3D<Cylindrical> {
        Vec3D::<Cylindrical>(
            [
                rho.abs(),
                if rho < 0.0 {
                    (phi + PI).fmod(PI2)
                } else {
                    phi.fmod(PI2)
                },
                z
            ],
            PhantomData::<Cylindrical>{}
        )
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
        let (r, mut phi, mut theta) = self.into();

        if rhs < 0.0 {
            phi += PI;
            theta = -theta;
        }

        Self::new(r * rhs.abs(), phi, theta)
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
        let (r, mut phi, mut theta) = self.into();

        if rhs < 0.0 {
            phi += PI;
            theta = -theta;
        }

        Self::new(r / rhs.abs(), phi, theta)
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

impl Vec3DNorm for Vec3D<Spherical> {
    fn norm(&self) -> f64 {
        self.0[0]
    }
}

impl Vec3D<Spherical> {
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

    pub fn new(r: f64, phi: f64, theta: f64) -> Vec3D<Spherical> {
        Vec3D::<Spherical>(
            [
                r.abs(),
                if r < 0.0 {
                    (phi + PI).fmod(PI2)
                } else {
                    phi.fmod(PI2)
                },
                Vec3D::<Spherical>::clamp(
                    if r < 0.0 {
                        -theta
                    } else {
                        theta
                    }
                )
            ],
            PhantomData::<Spherical>{}
        )
    }

    pub fn unit(phi: f64, theta: f64) -> Vec3D<Spherical> {
        Vec3D::<Spherical>(
            [
                1.0,
                phi.fmod(PI2),
                Vec3D::<Spherical>::clamp(theta)
            ],
            PhantomData::<Spherical>{}
        )
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CartesianVec3D {
    x: f64,
    y: f64,
    z: f64
}

impl convert::Into<(f64, f64, f64)> for CartesianVec3D {
    fn into(self) -> (f64, f64, f64) {
        let CartesianVec3D{ x, y, z } = self;
        (x, y, z)
    }
}

impl Vec3DNorm for CartesianVec3D {
    fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl CartesianVec3D {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CylindricalVec3D {
    rho: f64,
    phi: f64,
    z: f64
}

impl convert::Into<(f64, f64, f64)> for CylindricalVec3D {
    fn into(self) -> (f64, f64, f64) {
        let CylindricalVec3D { rho, phi, z } = self;
        (rho, phi, z)
    }
}

impl Vec3DNorm for CylindricalVec3D {
    fn norm(&self) -> f64 {
        self.rho.hypot(self.z)
    }
}

impl CylindricalVec3D {
    pub fn rho(&self) -> f64 {
        self.rho
    }

    pub fn phi(&self) -> f64 {
        self.phi
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SphericalVec3D {
    r: f64,
    phi: f64,
    theta: f64
}

impl convert::Into<(f64, f64, f64)> for SphericalVec3D {
    fn into(self) -> (f64, f64, f64) {
        let SphericalVec3D{ r, phi, theta } = self;
        (r, phi, theta)
    }
}

impl Vec3DNorm for SphericalVec3D {
    fn norm(&self) -> f64 {
        self.r
    }
}

impl SphericalVec3D {
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn phi(&self) -> f64 {
        self.phi
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Vector3D {
    Cartesian(CartesianVec3D),
    Cylindrical(CylindricalVec3D),
    Spherical(SphericalVec3D)
}

impl Vec3DNorm for Vector3D {
    fn norm(&self) -> f64 {
        match self {
            Vector3D::Cartesian(c) => c.norm(),
            Vector3D::Cylindrical(c) => c.norm(),
            Vector3D::Spherical(s) => s.norm()
        }
    }
}

impl default::Default for Vector3D {
    fn default() -> Self {
        Vector3D::zero()
    }
}

impl convert::Into<CartesianVec3D> for Vector3D {
    fn into(self) -> CartesianVec3D {
        match self {
            Vector3D::Cartesian(c) => c,
            Vector3D::Cylindrical(CylindricalVec3D{ rho, phi, z }) => {
                let (phi_sin, phi_cos) = phi.sin_cos();

                CartesianVec3D {
                    x: rho * phi_cos,
                    y: rho * phi_sin,
                    z
                }
            },
            Vector3D::Spherical(SphericalVec3D{ r, phi, theta }) => {
                let (phi_sin, phi_cos) = phi.sin_cos();
                let (theta_sin, theta_cos) = theta.sin_cos();
                let rho = r * theta_cos;

                CartesianVec3D {
                    x: rho * phi_cos,
                    y: rho * phi_sin,
                    z: r * theta_sin
                }
            }
        }
    }
}

impl convert::Into<Option<CartesianVec3D>> for Vector3D {
    fn into(self) -> Option<CartesianVec3D> {
        match self {
            Vector3D::Cartesian(c) => Some(c),
            _ => None
        }
    }
}

impl convert::Into<CylindricalVec3D> for Vector3D {
    fn into(self) -> CylindricalVec3D {
        match self {
            Vector3D::Cartesian(CartesianVec3D{ x, y, z }) => {
                let phi = if x == 0.0 && y == 0.0 {
                    0.0
                } else {
                    y.atan2(x)
                };

                CylindricalVec3D {
                    rho: x.hypot(y),
                    phi: phi.fmod(PI2),
                    z
                }
            },
            Vector3D::Cylindrical(c) => c,
            Vector3D::Spherical(SphericalVec3D{ r, phi, theta }) => {
                let (theta_sin, theta_cos) = theta.sin_cos();

                CylindricalVec3D {
                    rho: r * theta_cos,
                    phi,
                    z: r * theta_sin
                }
            }
        }
    }
}

impl convert::Into<Option<CylindricalVec3D>> for Vector3D {
    fn into(self) -> Option<CylindricalVec3D> {
        match self {
            Vector3D::Cylindrical(c) => Some(c),
            _ => None
        }
    }
}

impl convert::Into<SphericalVec3D> for Vector3D {
    fn into(self) -> SphericalVec3D {
        match self {
            Vector3D::Cartesian(CartesianVec3D{ x, y, z }) => {
                let rho_sq = x * x + y * y;
                let r = (rho_sq + z * z).sqrt();

                let phi = if x == 0.0 && y == 0.0 {
                    0.0
                } else {
                    y.atan2(x)
                };

                let rho = rho_sq.sqrt();
                let theta = if rho == 0.0 && z == 0.0 {
                    0.0
                } else {
                    z.atan2(rho)
                };

                SphericalVec3D { r, phi: phi.fmod(PI2), theta }
            },
            Vector3D::Cylindrical(CylindricalVec3D{ rho, phi, z }) => {
                let theta = if rho == 0.0 && z == 0.0 {
                    0.0
                } else {
                    z.atan2(rho)
                };

                SphericalVec3D { r: rho.hypot(z), phi, theta }
            },
            Vector3D::Spherical(s) => s
        }
    }
}

impl convert::Into<Option<SphericalVec3D>> for Vector3D {
    fn into(self) -> Option<SphericalVec3D> {
        match self {
            Vector3D::Spherical(s) => Some(s),
            _ => None
        }
    }
}

impl ops::Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self {
        let lhs: CartesianVec3D = self.into();
        Vector3D::from_c(-lhs.x, -lhs.y, -lhs.z)
    }
}

impl ops::Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vector3D::from_c(
            lhs.x + rhs.x,
            lhs.y + rhs.y,
            lhs.z + rhs.z
        )
    }
}

impl ops::AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.into();

        if let Vector3D::Cartesian(ref mut c) = self {
            c.x += rhs.x;
            c.y += rhs.y;
            c.z += rhs.z;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x += rhs.x;
            lhs.y += rhs.y;
            lhs.z += rhs.z;

            match *self {
                Vector3D::Cylindrical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_y();
                },
                Vector3D::Spherical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_s();
                },
                _ => unreachable!()
            }
        }
    }
}

impl ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vector3D::from_c(
            lhs.x - rhs.x,
            lhs.y - rhs.y,
            lhs.z - rhs.z
        )
    }
}

impl ops::SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.into();

        if let Vector3D::Cartesian(ref mut c) = self {
            c.x -= rhs.x;
            c.y -= rhs.y;
            c.z -= rhs.z;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x -= rhs.x;
            lhs.y -= rhs.y;
            lhs.z -= rhs.z;

            match *self {
                Vector3D::Cylindrical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_y();
                },
                Vector3D::Spherical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_s();
                },
                _ => unreachable!()
            }
        }
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let lhs: CartesianVec3D = self.into();

        Vector3D::from_c(
            lhs.x * rhs,
            lhs.y * rhs,
            lhs.z * rhs
        )
    }
}

impl ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        if let Vector3D::Cartesian(ref mut c) = self {
            c.x *= rhs;
            c.y *= rhs;
            c.z *= rhs;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x *= rhs;
            lhs.y *= rhs;
            lhs.z *= rhs;

            match *self {
                Vector3D::Cylindrical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_y();
                },
                Vector3D::Spherical(_) => {
                    *self = Vector3D::Cartesian(lhs).to_s();
                },
                _ => unreachable!()
            }
        }
    }
}

impl ops::Div<f64> for Vector3D {
    type Output = Result<Vector3D>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return Err(Error::ZeroDivisionError);
        }

        let lhs: CartesianVec3D = self.into();
        Ok(
            Vector3D::from_c(
                lhs.x / rhs,
                lhs.y / rhs,
                lhs.z / rhs
            )
        )
    }
}

impl ops::Div<Mat3D> for Vector3D {
    type Output = Result<Vector3D>;

    fn div(self, rhs: Mat3D) -> Self::Output {
        let inverted = rhs.inv()?;
        Ok(inverted.mul(self))
    }
}

impl Vector3D {
    pub fn from_c(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D::Cartesian(CartesianVec3D { x, y, z })
    }

    pub fn from_y(rho: f64, phi: f64, z: f64) -> Result<Vector3D> {
        if rho < 0.0 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("rho", rho)
            ));
        }

        Ok(
            Vector3D::Cylindrical(
                CylindricalVec3D { rho, phi: phi.fmod(PI2), z }
            )
        )
    }

    pub fn from_s(r: f64, phi: f64, theta: f64) -> Result<Vector3D> {
        if r < 0.0 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("r", r)
            ));
        }
        if theta < -FRAC_PI_2 || theta > FRAC_PI_2 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("theta", theta)
            ));
        }

        Ok(
            Vector3D::Spherical(
                SphericalVec3D { r, phi: phi.fmod(PI2), theta }
            )
        )
    }

    pub fn zero() -> Vector3D {
        Vector3D::Cartesian(
            CartesianVec3D { x: 0.0, y: 0.0, z: 0.0 }
        )
    }

    pub fn unit_x() -> Vector3D {
        Vector3D::Cartesian(
            CartesianVec3D { x: 1.0, y: 0.0, z: 0.0 }
        )
    }

    pub fn unit_y() -> Vector3D {
        Vector3D::Cartesian(
            CartesianVec3D { x: 0.0, y: 1.0, z: 0.0 }
        )
    }

    pub fn unit_z() -> Vector3D {
        Vector3D::Cartesian(
            CartesianVec3D { x: 0.0, y: 0.0, z: 1.0 }
        )
    }

    pub fn unit(phi: f64, theta: f64) -> Result<Vector3D> {
        if theta < -FRAC_PI_2 || theta > FRAC_PI_2 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("theta", theta)
            ));
        }

        Ok(
            Vector3D::Spherical(
                SphericalVec3D {
                    r: 1.0,
                    phi: phi.fmod(PI2),
                    theta
                }
            )
        )
    }

    pub fn to_c(self) -> Vector3D {
        Vector3D::Cartesian(self.into())
    }

    pub fn to_y(self) -> Vector3D {
        Vector3D::Cylindrical(self.into())
    }

    pub fn to_s(self) -> Vector3D {
        Vector3D::Spherical(self.into())
    }

    pub fn is_c(self) -> bool {
        match self {
            Vector3D::Cartesian(_) => true,
            _ => false
        }
    }

    pub fn is_y(self) -> bool {
        match self {
            Vector3D::Cylindrical(_) => true,
            _ => false
        }
    }

    pub fn is_s(self) -> bool {
        match self {
            Vector3D::Spherical(_) => true,
            _ => false
        }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Vector3D {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vector3D::from_c(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3D([[f64; 3]; 3]);

impl default::Default for Mat3D {
    fn default() -> Self {
        Mat3D::zeros()
    }
}

impl Vec3DNorm for Mat3D {
    fn norm(&self) -> f64 {
        let mut s = 0.0;

        for i in 0..3 {
            for j in 0..3 {
                s += self.0[i][j].powi(2);
            }
        }

        s.sqrt()
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

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] += rhs.0[i][j];
            }
        }

        result
    }
}

impl ops::AddAssign for Mat3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl ops::Sub for Mat3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = self;

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] -= rhs.0[i][j];
            }
        }

        result
    }
}

impl ops::SubAssign for Mat3D {
    fn sub_assign(&mut self, rhs: Mat3D) {
        *self = self.sub(rhs);
    }
}

impl ops::Mul<f64> for Mat3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = self;

        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] *= rhs;
            }
        }

        result
    }
}

impl ops::Mul<Mat3D> for f64 {
    type Output = Mat3D;

    fn mul(self, rhs: Mat3D) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Mat3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.mul(rhs);
    }
}

impl ops::Mul<Vector3D> for Mat3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let rhs: CartesianVec3D = rhs.into();

        let rhs = [rhs.x, rhs.y, rhs.z];
        let mut values = [0.0; 3];

        for i in 0..3 {
            for j in 0..3 {
                values[i] += self.0[i][j] * rhs[j];
            }
        }

        Vector3D::from_c(values[0], values[1], values[2])
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
    type Output = Result<Mat3D>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return Err(Error::ZeroDivisionError);
        }

        let mut result = self;
        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] /= rhs;
            }
        }

        Ok(result)
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

impl Mat3D {
    pub fn zeros() -> Mat3D {
        Mat3D([[0.0; 3]; 3])
    }

    pub fn ones() -> Mat3D {
        Mat3D([[1.0; 3]; 3])
    }

    pub fn identity() -> Mat3D {
        Mat3D(
            [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        )
    }

    pub fn from_rows(r1: Vector3D, r2: Vector3D, r3: Vector3D) -> Mat3D {
        let r1: CartesianVec3D = r1.into();
        let r2: CartesianVec3D = r2.into();
        let r3: CartesianVec3D = r3.into();

        Mat3D(
            [
                [r1.x, r1.y, r1.z],
                [r2.x, r2.y, r2.z],
                [r3.x, r3.y, r3.z]
            ]
        )
    }

    pub fn from_columns(c1: Vector3D, c2: Vector3D, c3: Vector3D) -> Mat3D {
        let c1: CartesianVec3D = c1.into();
        let c2: CartesianVec3D = c2.into();
        let c3: CartesianVec3D = c3.into();

        Mat3D(
            [
                [c1.x, c2.x, c3.x],
                [c1.y, c2.y, c3.y],
                [c1.z, c2.z, c3.z]
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
    fn wrap_index(index: isize) -> usize {
        let mut index = index % 3;
        if index < 0 {
            index += 3;
        }

        index as usize
    }

    pub fn row(&self, index: isize) -> Vector3D {
        let index = Mat3D::wrap_index(index);
        Vector3D::from_c(self.0[index][0], self.0[index][1], self.0[index][2])
    }

    pub fn column(&self, index: isize) -> Vector3D {
        let index = Mat3D::wrap_index(index);
        Vector3D::from_c(self.0[0][index], self.0[1][index], self.0[2][index])
    }

    pub fn t(&self) -> Mat3D {
        Mat3D(
            [
                [self.0[0][0], self.0[1][0], self.0[2][0]],
                [self.0[0][1], self.0[1][1], self.0[2][1]],
                [self.0[0][2], self.0[1][2], self.0[2][2]]
            ]
        )
    }

    pub fn tr(&self) -> f64 {
        self.0[0][0] + self.0[1][1] + self.0[2][2]
    }

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

    const EPS: f64 = 1e-10;
    const ITERATIONS: i32 = 200;

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
            for i in 0..2 {
                for j in 0..2 {
                    assert_eq!(m3.0[i][j], m1.0[i][j] - m2.0[i][j]);
                }
            }

            m1 -= m2;
            for i in 0..2 {
                for j in 0..2 {
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

        let c = (a / 3.0).unwrap();
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

            let c = a / k;
            match c {
                Ok(b) => {
                    for (v1, v2) in a.iter().zip(b.iter()) {
                        if k != 0.0 {
                            assert_relative_eq!(v1 / k, v2);
                        } else {
                            panic!("Illegal result for zero division");
                        }
                    }
                },
                Err(_) => {
                    assert_eq!(k, 0.0);
                }
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

        let indices: Vec<isize> = vec![0, 1, 2, -1, -2, -3];
        let matrix_indices: Vec<usize> = vec![0, 1, 2, 2, 1, 0];

        for (idx, matrix_idx) in indices.iter().zip(matrix_indices.iter()) {
            let row = a.row(*idx);
            if let Vector3D::Cartesian(CartesianVec3D{ x, y, z }) = row {
                assert_eq!(x, a.0[*matrix_idx][0]);
                assert_eq!(y, a.0[*matrix_idx][1]);
                assert_eq!(z, a.0[*matrix_idx][2]);
            } else {
                unreachable!();
            }

            let col = a.column(*idx);
            if let Vector3D::Cartesian(CartesianVec3D { x, y, z }) = col {
                assert_eq!(x, a.0[0][*matrix_idx]);
                assert_eq!(y, a.0[1][*matrix_idx]);
                assert_eq!(z, a.0[2][*matrix_idx]);
            } else {
                unreachable!();
            }
        }
    }
}
