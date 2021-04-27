use std::convert;
use std::f64::consts::FRAC_PI_2;

use crate::base::Real;
use crate::base::consts::PI2;
use crate::base::angle::{Angle, AngleMapper};
use crate::base::linalg::{Spherical, Vec3D};


// ########################################################
// # Equatorial coordinates
// ########################################################

pub struct Equatorial {
    right_ascension: f64,
    declination: f64,
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
    pub fn new(right_ascension: f64, declination: f64) -> Equatorial {
        Equatorial {
            right_ascension: right_ascension.fmod(PI2),
            declination: declination.clamp(-FRAC_PI_2, FRAC_PI_2)
        }
    }
}