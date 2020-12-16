use std::f64::consts::PI;
use std::ops;

use crate::base::consts::PI2;

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

#[derive(Debug, Copy, Clone)]
pub enum Vec3D {
    Cartesian(CartesianVec3D),
    Cylindrical(CylindricalVec3D),
    Spherical(SphericalVec3D)
}

impl Vec3D {
    pub fn new_cartesian(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D::Cartesian(CartesianVec3D { x, y, z })
    }

    pub fn new_cylindrical(rho: f64, phi: f64, z: f64) -> Vec3D {
        Vec3D::Cylindrical(CylindricalVec3D { rho, phi, z })
    }

    pub fn new_spherical(r: f64, phi: f64, theta: f64) -> Vec3D {
        Vec3D::Spherical(SphericalVec3D { r, phi, theta })
    }

    pub fn as_cartesian(&mut self) -> Option<&mut CartesianVec3D> {
        match *self {
            Vec3D::Cartesian(ref mut c) => Some(c),
            _ => None
        }
    }

    pub fn as_cylindrical(&mut self) -> Option<&mut CylindricalVec3D> {
        match *self {
            Vec3D::Cylindrical(ref mut c) => Some(c),
            _ => None
        }
    }

    pub fn as_spherical(&mut self) -> Option<&mut SphericalVec3D> {
        match *self {
            Vec3D::Spherical(ref mut s) => Some(s),
            _ => None
        }
    }

    pub fn to_cartesian(&self) -> CartesianVec3D {
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

    pub fn to_cylindrical(&self) -> CylindricalVec3D {
        match *self {
            Vec3D::Cartesian(ref c) => {
                let mut phi = if c.x == 0.0 && c.y == 0.0 {
                    0.0
                } else {
                    c.y.atan2(c.x)
                };
                if phi < 0.0 {
                    phi += PI2;
                }

                CylindricalVec3D {
                    rho: c.x.hypot(c.y),
                    phi,
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

    pub fn to_spherical(&self) -> SphericalVec3D {
        match *self {
            Vec3D::Cartesian(ref c) => {
                let rho_sq = c.x * c.x + c.y * c.y;
                let r = (rho_sq + c.z * c.z).sqrt();

                let mut phi = if c.x == 0.0 && c.y == 0.0 {
                    0.0
                } else {
                    c.y.atan2(c.x)
                };
                if phi < 0.0 {
                    phi += PI2;
                }

                let rho = rho_sq.sqrt();
                let theta = if rho == 0.0 && c.z == 0.0 {
                    0.0
                } else {
                    c.z.atan2(rho)
                };

                SphericalVec3D { r, phi, theta }
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

    pub fn dot(&self, rhs: &Self) -> f64 {
        let lhs = self.to_cartesian();
        let rhs = rhs.to_cartesian();

        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Vec3D {
        let lhs = self.to_cartesian();
        let rhs = rhs.to_cartesian();

        Vec3D::new_cartesian(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x
        )
    }
}

impl ops::Neg for Vec3D {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Vec3D::Cartesian(ref c) => {
                Vec3D::new_cartesian(-c.x, -c.y, -c.z)
            },
            Vec3D::Cylindrical(ref c) => {
                let mut phi = c.phi + PI;
                if phi > PI2 {
                    phi -= PI2;
                }

                Vec3D::new_cylindrical(c.rho, phi, -c.z)
            },
            Vec3D::Spherical(ref s) => {
                let mut phi = s.phi + PI;
                if phi > PI2 {
                    phi -= PI2;
                }

                Vec3D::new_spherical(s.r, phi, -s.theta)
            }
        }
    }
}

impl ops::Add for Vec3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs = self.to_cartesian();
        let rhs = rhs.to_cartesian();

        Vec3D::new_cartesian(
            lhs.x + rhs.x,
            lhs.y + rhs.y,
            lhs.z + rhs.z
        )
    }
}

impl ops::Sub for Vec3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs = self.to_cartesian();
        let rhs = rhs.to_cartesian();

        Vec3D::new_cartesian(
            lhs.x - rhs.x,
            lhs.y - rhs.y,
            lhs.z - rhs.z
        )
    }
}

impl ops::Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let lhs = self.to_cartesian();

        Vec3D::new_cartesian(
            lhs.x * rhs,
            lhs.y * rhs,
            lhs.z * rhs
        )
    }
}

impl ops::MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, rhs: f64) {
        match *self {
            Vec3D::Cartesian(ref mut c) => {
                c.x *= rhs;
                c.y *= rhs;
                c.z *= rhs;
            },
            Vec3D::Cylindrical(ref mut c) => {
                c.rho *= rhs;
                c.z *= rhs;
            },
            Vec3D::Spherical(ref mut s) => {
                s.r *= rhs;
            }
        }
    }
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
