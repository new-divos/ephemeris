use std::convert;
use std::default;

use crate::base::consts::{ARCS, MULT_2_PI, RAD};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}

trait AngleTools {
    fn sign(&self) -> Sign;
    fn normalize(&self) -> Self;
    fn value(&self) -> f64;

    fn units(&self) -> f64 {
        let mut value = self.value();
        if self.sign() == Sign::Negative {
            value = -value;
        }

        value
    }
}

impl AngleTools for (i32, f64) {
    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0.0 {
            Sign::Zero
        } else if self.0 < 0 || (self.0 == 0 && self.1 < 0.0) {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    fn normalize(&self) -> Self {
        let total = 60.0 * (self.0 as f64) + self.1;

        let t = total.abs();
        let mut value1 = (t / 60.0).floor();
        let mut value2 = t - 60.0 * value1;

        if value1 == 0.0 {
            value2 = value2.copysign(total);
        } else {
            value1 = value1.copysign(total);
        }

        (value1 as i32, value2)
    }

    fn value(&self) -> f64 {
        (self.0.abs() as f64) + self.1.abs() / 60.0
    }
}

impl AngleTools for (i32, i8, f64) {
    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0 && self.2 == 0.0 {
            Sign::Zero
        } else if self.0 < 0 || (
            self.0 == 0 && (self.1 < 0 || (self.1 == 0 && self.2 < 0.0))
        ) {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    fn normalize(&self) -> Self {
        let total = 60.0 * ((self.0 * 60 + self.1 as i32) as f64) + self.2;
        let t = total.abs();

        let value2 = (t / 60.0).floor();
        let mut value3 = t - 60.0 * value2;
        let mut value1 = (value2 / 60.0).floor();
        let mut value2 = value2 - 60.0 * value1;

        if value1 == 0.0 {
            if value2 == 0.0 {
                value3 = value3.copysign(total);
            } else {
                value2 = value2.copysign(total);
            }
        } else {
            value1 = value1.copysign(total);
        }

        (value1 as i32, value2 as i8, value3)
    }

    fn value(&self) -> f64 {
        (self.0.abs() as f64) + ((self.1.abs() as f64)
            + self.2.abs() / 60.0) / 60.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegreesMinutes {
    items: (i32, f64)
}

impl default::Default for AngleArcDegreesMinutes {
    fn default() -> Self {
        Self { items: (0, 0.0) }
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutes {
    fn into(self) -> f64 {
        self.items.units() * RAD
    }
}

impl AngleArcDegreesMinutes {
    pub fn degrees(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn minutes(&self) -> f64 {
        self.items.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(degrees: i32, minutes: f64) -> AngleArcDegreesMinutes {
        AngleArcDegreesMinutes { items: (degrees, minutes).normalize() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegreesMinutesSeconds {
    items: (i32, i8, f64)
}

impl default::Default for AngleArcDegreesMinutesSeconds {
    fn default() -> Self {
        Self { items: (0, 0, 0.0) }
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutesSeconds {
    fn into(self) -> f64 {
        self.items.units() * RAD
    }
}

impl AngleArcDegreesMinutesSeconds {
    pub fn degrees(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn minutes(&self) -> i32 {
        self.items.1.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        self.items.2.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(degrees: i32, minutes: i32, seconds: f64) -> AngleArcDegreesMinutesSeconds {
        let delta = minutes / 60;
        let degrees = degrees + delta;
        let minutes = (minutes - 60 * delta) as i8;

        AngleArcDegreesMinutesSeconds {
            items: (degrees, minutes, seconds).normalize()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcMinutesSeconds {
    items: (i32, f64)
}

impl convert::Into<f64> for AngleArcMinutesSeconds {
    fn into(self) -> f64 {
        (self.items.units() / 60.0) * RAD
    }
}

impl AngleArcMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn seconds(&self) -> f64 {
        self.items.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(minutes: i32, seconds: f64) -> AngleArcMinutesSeconds {
        AngleArcMinutesSeconds { items: (minutes, seconds).normalize() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHoursMinutes {
    items: (i32, f64)
}

impl default::Default for AngleTimeHoursMinutes {
    fn default() -> Self {
        Self { items: (0, 0.0) }
    }
}

impl convert::Into<f64> for AngleTimeHoursMinutes {
    fn into(self) -> f64 {
        (self.items.units() * 15.0) * RAD
    }
}

impl AngleTimeHoursMinutes {
    pub fn hours(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn minutes(&self) -> f64 {
        self.items.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(hours: i32, minutes: f64) -> AngleTimeHoursMinutes {
        AngleTimeHoursMinutes { items: (hours, minutes).normalize() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHoursMinutesSeconds {
    items: (i32, i8, f64)
}

impl default::Default for AngleTimeHoursMinutesSeconds {
    fn default() -> Self {
        Self { items: (0, 0, 0.0) }
    }
}

impl convert::Into<f64> for AngleTimeHoursMinutesSeconds {
    fn into(self) -> f64 {
        (self.items.units() * 15.0) * RAD
    }
}

impl AngleTimeHoursMinutesSeconds {
    pub fn hours(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn minutes(&self) -> i32 {
        self.items.1.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        self.items.2.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(hours: i32, minutes: i32, seconds: f64) -> AngleTimeHoursMinutesSeconds {
        let delta = minutes / 60;
        let hours = hours + delta;
        let minutes = (minutes - 60 * delta) as i8;

        AngleTimeHoursMinutesSeconds {
            items: (hours, minutes, seconds).normalize()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeMinutesSeconds {
    items: (i32, f64)
}

impl default::Default for AngleTimeMinutesSeconds {
    fn default() -> Self {
        Self { items: (0, 0.0) }
    }
}

impl convert::Into<f64> for AngleTimeMinutesSeconds {
    fn into(self) -> f64 {
        (self.items.units() / 4.0) * RAD
    }
}

impl AngleTimeMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        self.items.0.abs()
    }

    pub fn seconds(&self) -> f64 {
        self.items.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.items.sign()
    }

    fn new(minutes: i32, seconds: f64) -> AngleTimeMinutesSeconds {
        AngleTimeMinutesSeconds { items: (minutes, seconds).normalize() }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f64),
    Revolutions(f64),
    ArcDegrees(f64),
    ArcDegreesMinutes(AngleArcDegreesMinutes),
    ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds),
    ArcMinutes(f64),
    ArcMinutesSeconds(AngleArcMinutesSeconds),
    ArcSeconds(f64),
    TimeHours(f64),
    TimeHoursMinutes(AngleTimeHoursMinutes),
    TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds),
    TimeMinutes(f64),
    TimeMinutesSeconds(AngleTimeMinutesSeconds),
    TimeSeconds(f64)
}

impl default::Default for Angle {
    fn default() -> Self {
        Angle::Radians(0.0)
    }
}

impl convert::From<f64> for Angle {
    fn from(radians: f64) -> Self {
        Angle::Radians(radians)
    }
}

impl convert::Into<f64> for Angle {
    fn into(self) -> f64 {
        match self {
            Angle::Radians(r) => r,
            Angle::Revolutions(r) => MULT_2_PI * r,
            Angle::ArcDegrees(d) => RAD * d,
            Angle::ArcDegreesMinutes(dm) => dm.into(),
            Angle::ArcDegreesMinutesSeconds(dms) => dms.into(),
            Angle::ArcMinutes(m) => (m / 60.0) * RAD,
            Angle::ArcMinutesSeconds(ms) => ms.into(),
            Angle::ArcSeconds(s) => s / ARCS,
            Angle::TimeHours(h) => (h * 15.0) * RAD,
            Angle::TimeHoursMinutes(hm) => hm.into(),
            Angle::TimeHoursMinutesSeconds(hms) => hms.into(),
            Angle::TimeMinutes(m) => (m / 4.0) * RAD,
            Angle::TimeMinutesSeconds(ms) => ms.into(),
            Angle::TimeSeconds(s) => (s * 15.0) / ARCS
        }
    }
}

impl Angle {
    pub fn from_r(revolutions: f64) -> Angle {
        Angle::Revolutions(revolutions)
    }

    pub fn from_ad(degrees: f64) -> Angle {
        Angle::ArcDegrees(degrees)
    }

    pub fn from_adm(degrees: i32, minutes: f64) -> Angle {
        Angle::ArcDegreesMinutes(AngleArcDegreesMinutes::new(degrees, minutes))
    }

    pub fn from_adms(degrees: i32, minutes: i32, seconds: f64) -> Angle {
        Angle::ArcDegreesMinutesSeconds(
            AngleArcDegreesMinutesSeconds::new(degrees, minutes, seconds)
        )
    }

    pub fn from_am(minutes: f64) -> Angle {
        Angle::ArcMinutes(minutes)
    }

    pub fn from_ams(minutes: i32, seconds: f64) -> Angle {
        Angle::ArcMinutesSeconds(AngleArcMinutesSeconds::new(minutes, seconds))
    }

    pub fn from_as(seconds: f64) -> Angle {
        Angle::ArcSeconds(seconds)
    }

    pub fn from_th(hours: f64) -> Angle {
        Angle::TimeHours(hours)
    }

    pub fn from_thm(hours: i32, minutes: f64) -> Angle {
        Angle::TimeHoursMinutes(AngleTimeHoursMinutes::new(hours, minutes))
    }

    pub fn from_thms(hours: i32, minutes: i32, seconds: f64) -> Angle {
        Angle::TimeHoursMinutesSeconds(
            AngleTimeHoursMinutesSeconds::new(hours, minutes, seconds)
        )
    }

    pub fn from_tm(minutes: f64) -> Angle {
        Angle::TimeMinutes(minutes)
    }

    pub fn from_tms(minutes: i32, seconds: f64) -> Angle {
        Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds::new(minutes, seconds))
    }

    pub fn from_ts(seconds: f64) -> Angle {
        Angle::TimeSeconds(seconds)
    }
}