use std::convert;
use std::default;

use crate::base::consts::{ARCS, DEG, MULT_2_PI, RAD};

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

    fn from_units(units: f64) -> Self;

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

    fn from_units(units: f64) -> Self {
        let u = units.abs();
        let mut value1 = u.floor();
        let mut value2 = 60.0 * (u - value1);

        if value1 == 0.0 {
            value2 = value2.copysign(units);
        } else {
            value1 = value1.copysign(units);
        }

        (value1 as i32, value2)
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

    fn from_units(units: f64) -> Self {
        let u = units.abs();
        let mut value1 = u.floor();
        let value3 = 60.0 * (u - value1);
        let mut value2 = value3.floor();
        let mut value3 = 60.0 * (value3 - value2);

        if value1 == 0.0 {
            if value2 == 0.0 {
                value3 = value3.copysign(units);
            } else {
                value2 = value2.copysign(units);
            }
        } else {
            value1 = value1.copysign(units);
        }

        (value1 as i32, value2 as i8, value3)
    }
}

fn sign(angle: f64) -> Sign {
    if angle == 0.0 {
        Sign::Zero
    } else if angle < 0.0 {
        Sign::Negative
    } else {
        Sign::Positive
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRevolutions(f64);

impl default::Default for AngleRevolutions {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleRevolutions {
    fn into(self) -> f64 {
        MULT_2_PI * self.0
    }
}

impl AngleRevolutions {
    fn revolutions(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegrees(f64);

impl default::Default for AngleArcDegrees {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleArcDegrees {
    fn into(self) -> f64 {
        RAD * self.0
    }
}

impl AngleArcDegrees {
    fn degrees(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
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

    pub fn value(&self) -> f64 {
        self.items.units()
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

    pub fn value(&self) -> f64 {
        self.items.units()
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
pub struct AngleArcMinutes(f64);

impl default::Default for AngleArcMinutes {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleArcMinutes {
    fn into(self) -> f64 {
        RAD * (self.0 / 60.0)
    }
}

impl AngleArcMinutes {
    fn minutes(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
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

    pub fn value(&self) -> f64 {
        self.items.units()
    }

    fn new(minutes: i32, seconds: f64) -> AngleArcMinutesSeconds {
        AngleArcMinutesSeconds { items: (minutes, seconds).normalize() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcSeconds(f64);

impl default::Default for AngleArcSeconds {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleArcSeconds {
    fn into(self) -> f64 {
        self.0 / ARCS
    }
}

impl AngleArcSeconds {
    fn seconds(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHours(f64);

impl default::Default for AngleTimeHours {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleTimeHours {
    fn into(self) -> f64 {
        RAD * (self.0 * 15.0)
    }
}

impl AngleTimeHours {
    fn hours(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
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

    pub fn value(&self) -> f64 {
        self.items.units()
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

    pub fn value(&self) -> f64 {
        self.items.units()
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
pub struct AngleTimeMinutes(f64);

impl default::Default for AngleTimeMinutes {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleTimeMinutes {
    fn into(self) -> f64 {
        RAD * (self.0 / 4.0)
    }
}

impl AngleTimeMinutes {
    fn minutes(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
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

    pub fn value(&self) -> f64 {
        self.items.units()
    }

    fn new(minutes: i32, seconds: f64) -> AngleTimeMinutesSeconds {
        AngleTimeMinutesSeconds { items: (minutes, seconds).normalize() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeSeconds(f64);

impl default::Default for AngleTimeSeconds {
    fn default() -> Self {
        Self(0.0)
    }
}

impl convert::Into<f64> for AngleTimeSeconds {
    fn into(self) -> f64 {
        (self.0 * 15.0) / ARCS
    }
}

impl AngleTimeSeconds {
    fn seconds(&self) -> f64 {
        self.0.abs()
    }

    fn sign(&self) -> Sign {
        sign(self.0)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Angle {
    Radians(f64),
    Revolutions(AngleRevolutions),
    ArcDegrees(AngleArcDegrees),
    ArcDegreesMinutes(AngleArcDegreesMinutes),
    ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds),
    ArcMinutes(AngleArcMinutes),
    ArcMinutesSeconds(AngleArcMinutesSeconds),
    ArcSeconds(AngleArcSeconds),
    TimeHours(AngleTimeHours),
    TimeHoursMinutes(AngleTimeHoursMinutes),
    TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds),
    TimeMinutes(AngleTimeMinutes),
    TimeMinutesSeconds(AngleTimeMinutesSeconds),
    TimeSeconds(AngleTimeSeconds)
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
            Angle::Revolutions(r) => r.into(),
            Angle::ArcDegrees(d) => d.into(),
            Angle::ArcDegreesMinutes(dm) => dm.into(),
            Angle::ArcDegreesMinutesSeconds(dms) => dms.into(),
            Angle::ArcMinutes(m) => m.into(),
            Angle::ArcMinutesSeconds(ms) => ms.into(),
            Angle::ArcSeconds(s) => s.into(),
            Angle::TimeHours(h) => h.into(),
            Angle::TimeHoursMinutes(hm) => hm.into(),
            Angle::TimeHoursMinutesSeconds(hms) => hms.into(),
            Angle::TimeMinutes(m) => m.into(),
            Angle::TimeMinutesSeconds(ms) => ms.into(),
            Angle::TimeSeconds(s) => s.into()
        }
    }
}

impl convert::Into<AngleRevolutions> for Angle {
    fn into(self) -> AngleRevolutions {
        match self {
            Angle::Radians(r) => AngleRevolutions(r / MULT_2_PI),
            Angle::Revolutions(r) => r,
            Angle::ArcDegrees(d) => {
                AngleRevolutions(d.0 / 360.0)
            },
            Angle::ArcDegreesMinutes(dm) => {
                AngleRevolutions(dm.items.units() / 360.0)
            },
            Angle::ArcDegreesMinutesSeconds(dms) => {
                AngleRevolutions(dms.items.units() / 360.0)
            },
            Angle::ArcMinutes(m) => {
                AngleRevolutions(m.0 / (360.0 * 60.0))
            },
            Angle::ArcMinutesSeconds(ms) => {
                AngleRevolutions(ms.items.units() / (360.0 * 60.0))
            },
            Angle::ArcSeconds(s) => {
                AngleRevolutions(s.0 / (360.0 * 3600.0))
            },
            Angle::TimeHours(h) => {
                AngleRevolutions(h.0 / 24.0)
            },
            Angle::TimeHoursMinutes(hm) => {
                AngleRevolutions(hm.items.units() / 24.0)
            },
            Angle::TimeHoursMinutesSeconds(hms) => {
                AngleRevolutions(hms.items.units() / 24.0)
            },
            Angle::TimeMinutes(m) => {
                AngleRevolutions(m.0 / (24.0 * 60.0))
            },
            Angle::TimeMinutesSeconds(ms) => {
                AngleRevolutions(ms.items.units() / (24.0 * 60.0))
            },
            Angle::TimeSeconds(s) => {
                AngleRevolutions(s.0 / (24.0 * 3600.0))
            }
        }
    }
}

impl convert::Into<Option<AngleRevolutions>> for Angle {
    fn into(self) -> Option<AngleRevolutions> {
        match self {
            Angle::Revolutions(r) => Some(r),
            _ => None
        }
    }
}

impl convert::Into<AngleArcDegrees> for Angle {
    fn into(self) -> AngleArcDegrees {
        match self {
            Angle::Radians(r) => {
                AngleArcDegrees(DEG * r)
            },
            Angle::Revolutions(r) => {
                AngleArcDegrees(360.0 * r.0)
            },
            Angle::ArcDegrees(d) => d,
            Angle::ArcDegreesMinutes(dm) => {
                AngleArcDegrees(dm.items.units())
            },
            Angle::ArcDegreesMinutesSeconds(dms) => {
                AngleArcDegrees(dms.items.units())
            },
            Angle::ArcMinutes(m) => {
                AngleArcDegrees(m.0 / 60.0)
            },
            Angle::ArcMinutesSeconds(ms) => {
                AngleArcDegrees(ms.items.units() / 60.0)
            },
            Angle::ArcSeconds(s) => {
                AngleArcDegrees(s.0 / 3600.0)
            },
            Angle::TimeHours(h) => {
                AngleArcDegrees(h.0 * 15.0)
            },
            Angle::TimeHoursMinutes(hm) => {
                AngleArcDegrees(hm.items.units() * 15.0)
            },
            Angle::TimeHoursMinutesSeconds(hms) => {
                AngleArcDegrees(hms.items.units() * 15.0)
            },
            Angle::TimeMinutes(m) => {
                AngleArcDegrees(m.0 / 4.0)
            },
            Angle::TimeMinutesSeconds(ms) => {
                AngleArcDegrees(ms.items.units() / 4.0)
            },
            Angle::TimeSeconds(s) => {
                AngleArcDegrees(s.0 / 240.0)
            }
        }
    }
}

impl convert::Into<Option<AngleArcDegrees>> for Angle {
    fn into(self) -> Option<AngleArcDegrees> {
        match self {
            Angle::ArcDegrees(d) => Some(d),
            _ => None
        }
    }
}

impl Angle {
    pub fn from_r(revolutions: f64) -> Angle {
        Angle::Revolutions(AngleRevolutions(revolutions))
    }

    pub fn from_ad(degrees: f64) -> Angle {
        Angle::ArcDegrees(AngleArcDegrees(degrees))
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
        Angle::ArcMinutes(AngleArcMinutes(minutes))
    }

    pub fn from_ams(minutes: i32, seconds: f64) -> Angle {
        Angle::ArcMinutesSeconds(AngleArcMinutesSeconds::new(minutes, seconds))
    }

    pub fn from_as(seconds: f64) -> Angle {
        Angle::ArcSeconds(AngleArcSeconds(seconds))
    }

    pub fn from_th(hours: f64) -> Angle {
        Angle::TimeHours(AngleTimeHours(hours))
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
        Angle::TimeMinutes(AngleTimeMinutes(minutes))
    }

    pub fn from_tms(minutes: i32, seconds: f64) -> Angle {
        Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds::new(minutes, seconds))
    }

    pub fn from_ts(seconds: f64) -> Angle {
        Angle::TimeSeconds(AngleTimeSeconds(seconds))
    }

    pub fn is_rad(&self) -> bool {
        match self {
            Angle::Radians(_) => true,
            _ => false
        }
    }

    pub fn is_r(&self) -> bool {
        match self {
            Angle::Revolutions(_) => true,
            _ => false
        }
    }

    pub fn is_ad(&self) -> bool {
        match self {
            Angle::ArcDegrees(_) => true,
            _ => false
        }
    }

    pub fn to_rad(self) -> Angle {
        Angle::Radians(self.into())
    }

    pub fn to_r(self) -> Angle {
        Angle::Revolutions(self.into())
    }

    pub fn to_ad(self) -> Angle {
        Angle::ArcDegrees(self.into())
    }
}