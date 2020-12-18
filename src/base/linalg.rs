use std::f64::consts::FRAC_PI_2;
use std::ops;
use std::ops::{Add, Mul, Sub};

use crate::base::Modulo;
use crate::base::consts::MULT_2_PI;

pub trait Norm {
    fn norm(&self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CartesianVec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Norm for CartesianVec3D {
    fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CylindricalVec3D {
    pub rho: f64,
    pub phi: f64,
    pub z: f64
}

impl Norm for CylindricalVec3D {
    fn norm(&self) -> f64 {
        self.rho.hypot(self.z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SphericalVec3D {
    pub r: f64,
    pub phi: f64,
    pub theta: f64
}

impl Norm for SphericalVec3D {
    fn norm(&self) -> f64 {
        self.r
    }
}

pub trait Convert<T> {
    #[must_use]
    fn convert(&self) -> T;

    #[must_use]
    fn unwrap(&mut self) -> Option<&mut T>;
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

impl Convert<CartesianVec3D> for Vec3D {
    fn convert(&self) -> CartesianVec3D {
        match *self {
            Vec3D::Cartesian(ref c) => {
                c.clone()
            },
            Vec3D::Cylindrical(ref c) => {
                CartesianVec3D {
                    x: c.rho * c.phi.cos(),
                    y: c.rho * c.phi.sin(),
                    z: c.z
                }
            },
            Vec3D::Spherical(ref s) => {
                let rho = s.r * s.theta.cos();

                CartesianVec3D {
                    x: rho * s.phi.cos(),
                    y: rho * s.phi.sin(),
                    z: s.r * s.theta.sin()
                }
            }
        }
    }

    fn unwrap(&mut self) -> Option<&mut CartesianVec3D> {
        match *self {
            Vec3D::Cartesian(ref mut c) => Some(c),
            _ => None
        }
    }
}

impl Convert<CylindricalVec3D> for Vec3D {
    fn convert(&self) -> CylindricalVec3D {
        match *self {
            Vec3D::Cartesian(ref c) => {
                let phi = if c.x == 0.0 && c.y == 0.0 {
                    0.0
                } else {
                    c.y.atan2(c.x)
                };

                CylindricalVec3D {
                    rho: c.x.hypot(c.y),
                    phi: phi.modulo(MULT_2_PI),
                    z: c.z
                }
            },
            Vec3D::Cylindrical(ref c) => {
                c.clone()
            },
            Vec3D::Spherical(ref s) => {
                CylindricalVec3D {
                    rho: s.r * s.theta.cos(),
                    phi: s.phi,
                    z: s.r * s.theta.sin()
                }
            }
        }
    }

    fn unwrap(&mut self) -> Option<&mut CylindricalVec3D> {
        match *self {
            Vec3D::Cylindrical(ref mut c) => Some(c),
            _ => None
        }
    }
}

impl Convert<SphericalVec3D> for Vec3D {
    fn convert(&self) -> SphericalVec3D {
        match *self {
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

                SphericalVec3D { r, phi: phi.modulo(MULT_2_PI), theta }
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
            Vec3D::Spherical(ref s) => {
                s.clone()
            }
        }
    }

    fn unwrap(&mut self) -> Option<&mut SphericalVec3D> {
        match *self {
            Vec3D::Spherical(ref mut s) => Some(s),
            _ => None
        }
    }
}

impl ops::Neg for Vec3D {
    type Output = Self;

    fn neg(self) -> Self {
        let lhs: CartesianVec3D = self.convert();

        Vec3D::new_cartesian(-lhs.x, -lhs.y, -lhs.z)
    }
}

impl ops::Add for Vec3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs: CartesianVec3D = self.convert();
        let rhs: CartesianVec3D = rhs.convert();

        Vec3D::new_cartesian(
            lhs.x + rhs.x,
            lhs.y + rhs.y,
            lhs.z + rhs.z
        )
    }
}

impl ops::AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.convert();

        let opt: Option<&mut CartesianVec3D> = self.unwrap();
        if let Some(c) = opt {
            c.x += rhs.x;
            c.y += rhs.y;
            c.z += rhs.z;
        } else {
            let mut lhs: CartesianVec3D = self.convert();

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
        let lhs: CartesianVec3D = self.convert();
        let rhs: CartesianVec3D = rhs.convert();

        Vec3D::new_cartesian(
            lhs.x - rhs.x,
            lhs.y - rhs.y,
            lhs.z - rhs.z
        )
    }
}

impl ops::SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        let rhs: CartesianVec3D = rhs.convert();

        let opt: Option<&mut CartesianVec3D> = self.unwrap();
        if let Some(c) = opt {
            c.x -= rhs.x;
            c.y -= rhs.y;
            c.z -= rhs.z;
        } else {
            let mut lhs: CartesianVec3D = self.convert();

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
        let lhs: CartesianVec3D = self.convert();

        Vec3D::new_cartesian(
            lhs.x * rhs,
            lhs.y * rhs,
            lhs.z * rhs
        )
    }
}

impl ops::MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, rhs: f64) {
        let opt: Option<&mut CartesianVec3D> = self.unwrap();
        if let Some(c) = opt {
            c.x *= rhs;
            c.y *= rhs;
            c.z *= rhs;
        } else {
            let mut lhs: CartesianVec3D = self.convert();

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

impl Vec3D {
    pub fn new_cartesian(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D::Cartesian(CartesianVec3D { x, y, z })
    }

    pub fn new_cylindrical(rho: f64, phi: f64, z: f64) -> Option<Vec3D> {
        if rho < 0.0 {
            return None;
        }

        Some(
            Vec3D::Cylindrical(
                CylindricalVec3D {
                    rho,
                    phi: phi.modulo(MULT_2_PI),
                    z
                }
            )
        )
    }

    pub fn new_spherical(r: f64, phi: f64, theta: f64) -> Option<Vec3D> {
        if r < 0.0 || theta > FRAC_PI_2 || theta < -FRAC_PI_2 {
            return None;
        }

        Some(
            Vec3D::Spherical(
                SphericalVec3D {
                    r,
                    phi: phi.modulo(MULT_2_PI),
                    theta
                }
            )
        )
    }

    pub fn new_unit(phi: f64, theta: f64) -> Vec3D {
        Vec3D::Spherical(SphericalVec3D { r: 1.0, phi, theta })
    }

    pub fn to_cartesian(&self) -> Vec3D {
        Vec3D::Cartesian(self.convert())
    }

    pub fn to_cylindrical(&self) -> Vec3D {
        Vec3D::Cylindrical(self.convert())
    }

    pub fn to_spherical(&self) -> Vec3D {
        Vec3D::Spherical(self.convert())
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        let lhs: CartesianVec3D = self.convert();
        let rhs: CartesianVec3D = rhs.convert();

        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Vec3D {
        let lhs: CartesianVec3D = self.convert();
        let rhs: CartesianVec3D = rhs.convert();

        Vec3D::new_cartesian(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3D([[f64; 3]; 3]);

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

impl ops::MulAssign<f64> for Mat3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.mul(rhs);
    }
}

impl ops::Mul<Vec3D> for Mat3D {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        let rhs: CartesianVec3D = rhs.convert();

        let rhs = [rhs.x, rhs.y, rhs.z];
        let mut values = [0.0; 3];

        for i in 0..3 {
            for j in 0..3 {
                values[i] += self.0[i][j] * rhs[j];
            }
        }

        Vec3D::new_cartesian(
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

    pub fn from_rows(r1: &Vec3D, r2: &Vec3D, r3: &Vec3D) -> Mat3D {
        let r1: CartesianVec3D = r1.convert();
        let r2: CartesianVec3D = r2.convert();
        let r3: CartesianVec3D = r3.convert();

        Mat3D(
            [
                [r1.x, r1.y, r1.z],
                [r2.x, r2.y, r2.z],
                [r3.x, r3.y, r3.z]
            ]
        )
    }

    pub fn from_columns(c1: &Vec3D, c2: &Vec3D, c3: &Vec3D) -> Mat3D {
        let c1: CartesianVec3D = c1.convert();
        let c2: CartesianVec3D = c2.convert();
        let c3: CartesianVec3D = c3.convert();

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

    pub fn trans(&self) -> Mat3D {
        let mut result: Mat3D = *self;
        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = result.0[j][i];
            }
        }

        result
    }

    pub fn det(&self) -> f64 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] -
                self.0[1][2] * self.0[2][1]) -
            self.0[0][1] * (self.0[1][0] * self.0[2][2] -
                self.0[1][2] * self.0[2][0]) +
            self.0[0][2] * (self.0[1][0] * self.0[2][1] -
                self.0[1][1] * self.0[2][0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
