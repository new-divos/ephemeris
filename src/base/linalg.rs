use std::f64::consts::FRAC_PI_2;
use std::convert;
use std::default;
use std::iter;
use std::ops;
use std::ops::{Add, Mul, Sub};

use crate::base::{Real, Result};
use crate::base::consts::PI2;
use crate::base::error::Error;

pub trait Norm {
    fn norm(&self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CartesianVec3D {
    x: f64,
    y: f64,
    z: f64
}

impl Norm for CartesianVec3D {
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

impl Norm for CylindricalVec3D {
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

impl Norm for SphericalVec3D {
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
pub enum Vec3D {
    Cartesian(CartesianVec3D),
    Cylindrical(CylindricalVec3D),
    Spherical(SphericalVec3D)
}

impl Norm for Vec3D {
    fn norm(&self) -> f64 {
        match *self {
            Vec3D::Cartesian(ref c) => c.norm(),
            Vec3D::Cylindrical(ref c) => c.norm(),
            Vec3D::Spherical(ref s) => s.norm()
        }
    }
}

impl default::Default for Vec3D {
    fn default() -> Self {
        Vec3D::zero()
    }
}

impl convert::Into<CartesianVec3D> for Vec3D {
    fn into(self) -> CartesianVec3D {
        match self {
            Vec3D::Cartesian(c) => c,
            Vec3D::Cylindrical(ref c) => {
                let p = c.phi.sin_cos();

                CartesianVec3D {
                    x: c.rho * p.1,
                    y: c.rho * p.0,
                    z: c.z
                }
            },
            Vec3D::Spherical(ref s) => {
                let p = s.phi.sin_cos();
                let t = s.theta.sin_cos();
                let rho = s.r * t.1;

                CartesianVec3D {
                    x: rho * p.1,
                    y: rho * p.0,
                    z: s.r * t.0
                }
            }
        }
    }
}

impl convert::Into<Option<CartesianVec3D>> for Vec3D {
    fn into(self) -> Option<CartesianVec3D> {
        match self {
            Vec3D::Cartesian(c) => Some(c),
            _ => None
        }
    }
}

impl convert::Into<CylindricalVec3D> for Vec3D {
    fn into(self) -> CylindricalVec3D {
        match self {
            Vec3D::Cartesian(ref c) => {
                let phi = if c.x == 0.0 && c.y == 0.0 {
                    0.0
                } else {
                    c.y.atan2(c.x)
                };

                CylindricalVec3D {
                    rho: c.x.hypot(c.y),
                    phi: phi.fmod(PI2),
                    z: c.z
                }
            },
            Vec3D::Cylindrical(c) => c,
            Vec3D::Spherical(ref s) => {
                let t = s.theta.sin_cos();

                CylindricalVec3D {
                    rho: s.r * t.1,
                    phi: s.phi,
                    z: s.r * t.0
                }
            }
        }
    }
}

impl convert::Into<Option<CylindricalVec3D>> for Vec3D {
    fn into(self) -> Option<CylindricalVec3D> {
        match self {
            Vec3D::Cylindrical(c) => Some(c),
            _ => None
        }
    }
}

impl convert::Into<SphericalVec3D> for Vec3D {
    fn into(self) -> SphericalVec3D {
        match self {
            Vec3D::Cartesian(ref c) => {
                let rho_sq = c.x.powi(2) + c.y.powi(2);
                let r = (rho_sq + c.z.powi(2)).sqrt();

                let phi = if c.x == 0.0 && c.y == 0.0 {
                    0.0
                } else {
                    c.y.atan2(c.x)
                };

                let rho = rho_sq.sqrt();
                let theta = if rho == 0.0 && c.z == 0.0 {
                    0.0
                } else {
                    c.z.atan2(rho)
                };

                SphericalVec3D { r, phi: phi.fmod(PI2), theta }
            },
            Vec3D::Cylindrical(ref c) => {
                let theta = if c.rho == 0.0 && c.z == 0.0 {
                    0.0
                } else {
                    c.z.atan2(c.rho)
                };

                SphericalVec3D {
                    r: c.rho.hypot(c.z),
                    phi: c.phi,
                    theta
                }
            },
            Vec3D::Spherical(s) => s
        }
    }
}

impl convert::Into<Option<SphericalVec3D>> for Vec3D {
    fn into(self) -> Option<SphericalVec3D> {
        match self {
            Vec3D::Spherical(s) => Some(s),
            _ => None
        }
    }
}

impl ops::Neg for Vec3D {
    type Output = Self;

    fn neg(self) -> Self {
        let lhs: CartesianVec3D = self.into();
        Vec3D::cartesian(-lhs.x, -lhs.y, -lhs.z)
    }
}

impl ops::Add for Vec3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vec3D::cartesian(
            lhs.x + rhs.x,
            lhs.y + rhs.y,
            lhs.z + rhs.z
        )
    }
}

impl ops::AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.into();

        if let Vec3D::Cartesian(ref mut c) = self {
            c.x += rhs.x;
            c.y += rhs.y;
            c.z += rhs.z;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x += rhs.x;
            lhs.y += rhs.y;
            lhs.z += rhs.z;

            match *self {
                Vec3D::Cylindrical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_cylindrical();
                },
                Vec3D::Spherical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_spherical();
                },
                _ => panic!(
                        "Invalid Vec3D conversion for addition with assignment"
                    )
            }
        }
    }
}

impl ops::Sub for Vec3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vec3D::cartesian(
            lhs.x - rhs.x,
            lhs.y - rhs.y,
            lhs.z - rhs.z
        )
    }
}

impl ops::SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.into();

        if let Vec3D::Cartesian(ref mut c) = self {
            c.x -= rhs.x;
            c.y -= rhs.y;
            c.z -= rhs.z;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x -= rhs.x;
            lhs.y -= rhs.y;
            lhs.z -= rhs.z;

            match *self {
                Vec3D::Cylindrical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_cylindrical();
                },
                Vec3D::Spherical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_spherical();
                },
                _ => panic!(
                    "Invalid Vec3D conversion for subtraction with assignment"
                )
            }
        }
    }
}

impl ops::Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let lhs: CartesianVec3D = self.into();

        Vec3D::cartesian(
            lhs.x * rhs,
            lhs.y * rhs,
            lhs.z * rhs
        )
    }
}

impl ops::Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, rhs: f64) {
        if let Vec3D::Cartesian(ref mut c) = self {
            c.x *= rhs;
            c.y *= rhs;
            c.z *= rhs;
        } else {
            let mut lhs: CartesianVec3D = (*self).into();

            lhs.x *= rhs;
            lhs.y *= rhs;
            lhs.z *= rhs;

            match *self {
                Vec3D::Cylindrical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_cylindrical();
                },
                Vec3D::Spherical(_) => {
                    *self = Vec3D::Cartesian(lhs).to_spherical();
                },
                _ => panic!(
                    "Invalid Vec3D conversion for multiplication with assignment"
                )
            }
        }
    }
}

impl ops::Div<f64> for Vec3D {
    type Output = Result<Vec3D>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return Err(Error::ZeroDivisionError);
        }

        let lhs: CartesianVec3D = self.into();
        Ok(
            Vec3D::cartesian(
                lhs.x / rhs,
                lhs.y / rhs,
                lhs.z / rhs
            )
        )
    }
}

impl ops::Div<Mat3D> for Vec3D {
    type Output = Result<Vec3D>;

    fn div(self, rhs: Mat3D) -> Self::Output {
        let inverted = rhs.inv()?;
        Ok(inverted.mul(self))
    }
}

impl Vec3D {
    pub fn cartesian(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D::Cartesian(CartesianVec3D { x, y, z })
    }

    pub fn cylindrical(rho: f64, phi: f64, z: f64) -> Result<Vec3D> {
        if rho < 0.0 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("rho", rho)
            ));
        }

        Ok(
            Vec3D::Cylindrical(
                CylindricalVec3D {
                    rho,
                    phi: phi.fmod(PI2),
                    z
                }
            )
        )
    }

    pub fn spherical(r: f64, phi: f64, theta: f64) -> Result<Vec3D> {
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
            Vec3D::Spherical(
                SphericalVec3D {
                    r,
                    phi: phi.fmod(PI2),
                    theta
                }
            )
        )
    }

    pub fn zero() -> Vec3D {
        Vec3D::Cartesian(
            CartesianVec3D {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        )
    }

    pub fn unit_x() -> Vec3D {
        Vec3D::Cartesian(
            CartesianVec3D {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
        )
    }

    pub fn unit_y() -> Vec3D {
        Vec3D::Cartesian(
            CartesianVec3D {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        )
    }

    pub fn unit_z() -> Vec3D {
        Vec3D::Cartesian(
            CartesianVec3D {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        )
    }

    pub fn unit(phi: f64, theta: f64) -> Result<Vec3D> {
        if theta < -FRAC_PI_2 || theta > FRAC_PI_2 {
            return Err(Error::CannotCreateVec3DError(
                Error::new_attribute_info("theta", theta)
            ));
        }

        Ok(
            Vec3D::Spherical(
                SphericalVec3D {
                    r: 1.0,
                    phi: phi.fmod(PI2),
                    theta
                }
            )
        )
    }

    pub fn to_cartesian(self) -> Vec3D {
        Vec3D::Cartesian(self.into())
    }

    pub fn to_cylindrical(self) -> Vec3D {
        Vec3D::Cylindrical(self.into())
    }

    pub fn to_spherical(self) -> Vec3D {
        Vec3D::Spherical(self.into())
    }

    pub fn is_cartesian(self) -> bool {
        match self {
            Vec3D::Cartesian(_) => true,
            _ => false
        }
    }

    pub fn is_cylindrical(self) -> bool {
        match self {
            Vec3D::Cylindrical(_) => true,
            _ => false
        }
    }

    pub fn is_spherical(self) -> bool {
        match self {
            Vec3D::Spherical(_) => true,
            _ => false
        }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Vec3D {
        let lhs: CartesianVec3D = self.into();
        let rhs: CartesianVec3D = rhs.into();

        Vec3D::cartesian(
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

impl Norm for Mat3D {
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

impl ops::Mul<Vec3D> for Mat3D {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        let rhs: CartesianVec3D = rhs.into();

        let rhs = [rhs.x, rhs.y, rhs.z];
        let mut values = [0.0; 3];

        for i in 0..3 {
            for j in 0..3 {
                values[i] += self.0[i][j] * rhs[j];
            }
        }

        Vec3D::cartesian(
            values[0],
            values[1],
            values[2]
        )
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

    pub fn from_rows(r1: Vec3D, r2: Vec3D, r3: Vec3D) -> Mat3D {
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

    pub fn from_columns(c1: Vec3D, c2: Vec3D, c3: Vec3D) -> Mat3D {
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
        let v = angle.sin_cos();

        Mat3D(
            [
                [1.0,  0.0, 0.0],
                [0.0,  v.1, v.0],
                [0.0, -v.0, v.1]
            ]
        )
    }

    pub fn r_y(angle: f64) -> Mat3D {
        let v = angle.sin_cos();

        Mat3D(
            [
                [v.1, 0.0, -v.0],
                [0.0, 1.0,  0.0],
                [v.0, 0.0,  v.1]
            ]
        )
    }

    pub fn r_z(angle: f64) -> Mat3D {
        let v = angle.sin_cos();

        Mat3D(
            [
                [ v.1, v.0, 0.0],
                [-v.0, v.1, 0.0],
                [ 0.0, 0.0, 1.0]
            ]
        )
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
}
