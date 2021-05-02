use std::convert;
use std::f64::consts::FRAC_PI_2;

use crate::base::Real;
use crate::base::consts::PI2;
use crate::base::linalg::{Spherical, Vec3D};


// ########################################################
// # Equatorial coordinates
// ########################################################

#[derive(Copy, Clone)]
pub struct Equatorial {
    right_ascension: f64,
    declination: f64,
}

impl convert::From<Vec3D<Spherical>> for Equatorial {
    #[inline]
    fn from(vector: Vec3D<Spherical>) -> Self {
        Self::new(vector.azimuth(), vector.colatitude())
    }
}

impl convert::Into<Vec3D<Spherical>> for Equatorial {
    #[inline]
    fn into(self) -> Vec3D<Spherical> {
        Vec3D::<Spherical>::unit(self.right_ascension, self.declination)
    }
}

impl Equatorial {
    #[inline]
    pub fn right_ascension(&self) -> f64 {
        self.right_ascension
    }

    #[inline]
    pub fn declination(&self) -> f64 {
        self.declination
    }

    #[inline]
    pub fn hour_angle(&self, sidereal_time: f64) -> f64 {
        (sidereal_time - self.right_ascension).fmod(PI2)
    }

    #[inline]
    pub fn new(right_ascension: f64, declination: f64) -> Self {
        Self {
            right_ascension: right_ascension.fmod(PI2),
            declination: declination.clamp(-FRAC_PI_2, FRAC_PI_2)
        }
    }

    #[inline]
    pub fn with_hour_angle(sidereal_time: f64, hour_angle: f64, declination: f64) -> Self {
        Self {
            right_ascension: (sidereal_time - hour_angle).fmod(PI2),
            declination: declination.clamp(-FRAC_PI_2, FRAC_PI_2)
        }
    }
}