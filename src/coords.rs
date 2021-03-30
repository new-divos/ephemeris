use std::convert;

use crate::base::angle::{Angle, AngleMapper};
use crate::base::linalg::{Spherical, Vec3D};


#[derive(Clone, Copy)]
pub struct Equatorial<RAT=f64, DT=f64>
    where
        DT: Copy,
        RAT: Copy
{
    right_ascension: RAT,
    declination: DT
}

impl<U, V> convert::From<Equatorial<f64, f64>> for Equatorial<Angle<U>, Angle<V>>
    where
        U: AngleMapper + Copy,
        Angle<U>: convert::From<f64>,
        V: AngleMapper + Copy,
        Angle<V>: convert::From<f64>
{
    #[inline]
    fn from(raw: Equatorial<f64, f64>) -> Self {
        Self {
            right_ascension: Angle::<U>::from(raw.right_ascension),
            declination: Angle::<V>::from(raw.declination)
        }
    }
}

impl<U, V> convert::From<Equatorial<Angle<U>, Angle<V>>> for Equatorial<f64, f64>
    where
        U: AngleMapper + Copy,
        Angle<U>: convert::Into<f64>,
        V: AngleMapper + Copy,
        Angle<V>: convert::Into<f64>
{
    #[inline]
    fn from(c: Equatorial<Angle<U>, Angle<V>>) -> Self {
        Self {
            right_ascension: c.right_ascension.into(),
            declination: c.declination.into()
        }
    }
}

impl convert::From<Equatorial<f64, f64>> for Vec3D<Spherical> {
    #[inline]
    fn from(c: Equatorial<f64, f64>) -> Self {
        Self::unit(c.right_ascension, c.declination)
    }
}

impl convert::From<Vec3D<Spherical>> for Equatorial<f64, f64> {
    #[inline]
    fn from(v: Vec3D<Spherical>) -> Self {
        Self {
            right_ascension: v.azimuth(),
            declination: v.colatitude()
        }
    }
}

impl<U, V> convert::From<Equatorial<Angle<U>, Angle<V>>> for Vec3D<Spherical>
    where
        U: AngleMapper + Copy,
        Angle<U>: convert::Into<f64>,
        V: AngleMapper + Copy,
        Angle<V>: convert::Into<f64>
{
    #[inline]
    fn from(c: Equatorial<Angle<U>, Angle<V>>) -> Self {
        Self::unit(
            c.right_ascension.into(),
            c.declination.into()
        )
    }
}

impl<U, V> convert::From<Vec3D<Spherical>> for Equatorial<Angle<U>, Angle<V>>
    where
        U: AngleMapper + Copy,
        Angle<U>: convert::From<f64>,
        V: AngleMapper + Copy,
        Angle<V>: convert::From<f64>
{
    #[inline]
    fn from(v: Vec3D<Spherical>) -> Self {
        Self {
            right_ascension: Angle::<U>::from(v.azimuth()),
            declination: Angle::<V>::from(v.colatitude())
        }
    }
}

impl<RAT: Copy, DT: Copy> Equatorial<RAT, DT> {
    #[inline]
    pub fn right_ascension(&self) -> RAT {
        self.right_ascension
    }

    #[inline]
    pub fn declination(&self) -> DT {
        self.declination
    }

    #[inline]
    pub fn new(right_ascension: RAT, declination: DT) -> Self {
        Self { right_ascension, declination }
    }
}