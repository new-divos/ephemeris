use std::convert;
use std::default;

use crate::base::consts::{ARCS, DEG, MULT_2_PI, RAD};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}

trait AngleSign {
    fn sign(&self) -> Sign;
}

impl AngleSign for f64 {
    fn sign(&self) -> Sign {
        if *self == 0.0 {
            Sign::Zero
        } else if *self < 0.0 {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }
}

trait AngleValue: AngleSign {
    fn normalize(&self) -> Self;
    fn units(&self) -> f64;

    fn value(&self) -> f64 {
        let mut value = self.units();
        if (*self).sign() == Sign::Negative {
            value = -value;
        }

        value
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct ShortAngle(i32, f64);

impl default::Default for ShortAngle {
    fn default() -> Self {
        Self(0, 0.0)
    }
}

impl convert::Into<ShortAngle> for f64 {
    fn into(self) -> ShortAngle {
        let u = self.abs();
        let mut value1 = u.floor();
        let mut value2 = 60.0 * (u - value1);

        if value1 == 0.0 {
            value2 = value2.copysign(self);
        } else {
            value1 = value1.copysign(self);
        }

        ShortAngle(value1 as i32, value2)
    }
}

impl AngleSign for ShortAngle {
    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0.0 {
            Sign::Zero
        } else if self.0 < 0 || (self.0 == 0 && self.1 < 0.0) {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }
}

impl AngleValue for ShortAngle {
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

        ShortAngle(value1 as i32, value2)
    }

    fn units(&self) -> f64 {
        (self.0.abs() as f64) + self.1.abs() / 60.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct LongAngle(i32, i8, f64);

impl default::Default for LongAngle {
    fn default() -> Self {
        Self(0, 0, 0.0)
    }
}

impl convert::Into<LongAngle> for f64 {
    fn into(self) -> LongAngle {
        let u = self.abs();
        let mut value1 = u.floor();
        let value3 = 60.0 * (u - value1);
        let mut value2 = value3.floor();
        let mut value3 = 60.0 * (value3 - value2);

        if value1 == 0.0 {
            if value2 == 0.0 {
                value3 = value3.copysign(self);
            } else {
                value2 = value2.copysign(self);
            }
        } else {
            value1 = value1.copysign(self);
        }

        LongAngle(value1 as i32, value2 as i8, value3)
    }
}

impl AngleSign for LongAngle {
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
}

impl AngleValue for LongAngle {
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

        LongAngle(value1 as i32, value2 as i8, value3)
    }

    fn units(&self) -> f64 {
        (self.0.abs() as f64) + ((self.1.abs() as f64)
            + self.2.abs() / 60.0) / 60.0
    }
}

macro_rules! impl_default {
    ($t:ty) => (
        impl default::Default for $t {
            fn default() -> Self {
                Self(0.0)
            }
        }
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRevolutions(f64);

impl_default!(AngleRevolutions);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegrees(f64);

impl_default!(AngleArcDegrees);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegreesMinutes(ShortAngle);

impl default::Default for AngleArcDegreesMinutes {
    fn default() -> Self {
        Self(ShortAngle::default())
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutes {
    fn into(self) -> f64 {
        self.0.value() * RAD
    }
}

impl AngleArcDegreesMinutes {
    pub fn degrees(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn minutes(&self) -> f64 {
        self.0.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(degrees: i32, minutes: f64) -> AngleArcDegreesMinutes {
        AngleArcDegreesMinutes(ShortAngle(degrees, minutes).normalize())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegreesMinutesSeconds(LongAngle);

impl default::Default for AngleArcDegreesMinutesSeconds {
    fn default() -> Self {
        Self(LongAngle::default())
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutesSeconds {
    fn into(self) -> f64 {
        self.0.value() * RAD
    }
}

impl AngleArcDegreesMinutesSeconds {
    pub fn degrees(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn minutes(&self) -> i32 {
        self.0.1.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        self.0.2.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(degrees: i32, minutes: i32, seconds: f64) -> AngleArcDegreesMinutesSeconds {
        let delta = minutes / 60;
        let degrees = degrees + delta;
        let minutes = (minutes - 60 * delta) as i8;

        AngleArcDegreesMinutesSeconds(
            LongAngle(degrees, minutes, seconds).normalize()
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcMinutes(f64);

impl_default!(AngleArcMinutes);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcMinutesSeconds(ShortAngle);

impl convert::Into<f64> for AngleArcMinutesSeconds {
    fn into(self) -> f64 {
        (self.0.value() / 60.0) * RAD
    }
}

impl AngleArcMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn seconds(&self) -> f64 {
        self.0.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(minutes: i32, seconds: f64) -> AngleArcMinutesSeconds {
        AngleArcMinutesSeconds(ShortAngle(minutes, seconds).normalize())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcSeconds(f64);

impl_default!(AngleArcSeconds);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHours(f64);

impl_default!(AngleTimeHours);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHoursMinutes(ShortAngle);

impl default::Default for AngleTimeHoursMinutes {
    fn default() -> Self {
        Self(ShortAngle::default())
    }
}

impl convert::Into<f64> for AngleTimeHoursMinutes {
    fn into(self) -> f64 {
        (self.0.value() * 15.0) * RAD
    }
}

impl AngleTimeHoursMinutes {
    pub fn hours(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn minutes(&self) -> f64 {
        self.0.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(hours: i32, minutes: f64) -> AngleTimeHoursMinutes {
        AngleTimeHoursMinutes(ShortAngle(hours, minutes).normalize())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeHoursMinutesSeconds(LongAngle);

impl default::Default for AngleTimeHoursMinutesSeconds {
    fn default() -> Self {
        Self(LongAngle::default())
    }
}

impl convert::Into<f64> for AngleTimeHoursMinutesSeconds {
    fn into(self) -> f64 {
        (self.0.value() * 15.0) * RAD
    }
}

impl AngleTimeHoursMinutesSeconds {
    pub fn hours(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn minutes(&self) -> i32 {
        self.0.1.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        self.0.2.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(hours: i32, minutes: i32, seconds: f64) -> AngleTimeHoursMinutesSeconds {
        let delta = minutes / 60;
        let hours = hours + delta;
        let minutes = (minutes - 60 * delta) as i8;

        AngleTimeHoursMinutesSeconds(
            LongAngle(hours, minutes, seconds).normalize()
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeMinutes(f64);

impl_default!(AngleTimeMinutes);

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
        self.0.sign()
    }

    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeMinutesSeconds(ShortAngle);

impl default::Default for AngleTimeMinutesSeconds {
    fn default() -> Self {
        Self(ShortAngle::default())
    }
}

impl convert::Into<f64> for AngleTimeMinutesSeconds {
    fn into(self) -> f64 {
        (self.0.value() / 4.0) * RAD
    }
}

impl AngleTimeMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        self.0.0.abs()
    }

    pub fn seconds(&self) -> f64 {
        self.0.1.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }

    fn new(minutes: i32, seconds: f64) -> AngleTimeMinutesSeconds {
        AngleTimeMinutesSeconds(ShortAngle(minutes, seconds).normalize())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleTimeSeconds(f64);

impl_default!(AngleTimeSeconds);

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
        self.0.sign()
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
                AngleRevolutions(dm.0.value() / 360.0)
            },
            Angle::ArcDegreesMinutesSeconds(dms) => {
                AngleRevolutions(dms.0.value() / 360.0)
            },
            Angle::ArcMinutes(m) => {
                AngleRevolutions(m.0 / (360.0 * 60.0))
            },
            Angle::ArcMinutesSeconds(ms) => {
                AngleRevolutions(ms.0.value() / (360.0 * 60.0))
            },
            Angle::ArcSeconds(s) => {
                AngleRevolutions(s.0 / (360.0 * 3600.0))
            },
            Angle::TimeHours(h) => {
                AngleRevolutions(h.0 / 24.0)
            },
            Angle::TimeHoursMinutes(hm) => {
                AngleRevolutions(hm.0.value() / 24.0)
            },
            Angle::TimeHoursMinutesSeconds(hms) => {
                AngleRevolutions(hms.0.value() / 24.0)
            },
            Angle::TimeMinutes(m) => {
                AngleRevolutions(m.0 / (24.0 * 60.0))
            },
            Angle::TimeMinutesSeconds(ms) => {
                AngleRevolutions(ms.0.value() / (24.0 * 60.0))
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
                AngleArcDegrees(dm.0.value())
            },
            Angle::ArcDegreesMinutesSeconds(dms) => {
                AngleArcDegrees(dms.0.value())
            },
            Angle::ArcMinutes(m) => {
                AngleArcDegrees(m.0 / 60.0)
            },
            Angle::ArcMinutesSeconds(ms) => {
                AngleArcDegrees(ms.0.value() / 60.0)
            },
            Angle::ArcSeconds(s) => {
                AngleArcDegrees(s.0 / 3600.0)
            },
            Angle::TimeHours(h) => {
                AngleArcDegrees(h.0 * 15.0)
            },
            Angle::TimeHoursMinutes(hm) => {
                AngleArcDegrees(hm.0.value() * 15.0)
            },
            Angle::TimeHoursMinutesSeconds(hms) => {
                AngleArcDegrees(hms.0.value() * 15.0)
            },
            Angle::TimeMinutes(m) => {
                AngleArcDegrees(m.0 / 4.0)
            },
            Angle::TimeMinutesSeconds(ms) => {
                AngleArcDegrees(ms.0.value() / 4.0)
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

impl convert::Into<AngleArcDegreesMinutes> for Angle {
    fn into(self) -> AngleArcDegreesMinutes {
        match self {
            Angle::Radians(r) => {
                AngleArcDegreesMinutes((DEG * r).into())
            },
            Angle::Revolutions(r) => {
                AngleArcDegreesMinutes((360.0 * r.0).into())
            },
            Angle::ArcDegrees(d) => {
                AngleArcDegreesMinutes(d.0.into())
            },
            Angle::ArcDegreesMinutes(dm) => dm,
            Angle::ArcDegreesMinutesSeconds(dms) => {
                let ms = ShortAngle(dms.0.1 as i32, dms.0.2);
                AngleArcDegreesMinutes(ShortAngle(dms.0.0, ms.value()))
            },
            Angle::ArcMinutes(m) => {
                AngleArcDegreesMinutes(ShortAngle(0, m.0).normalize())
            },
            Angle::ArcMinutesSeconds(ms) => {
                AngleArcDegreesMinutes(ShortAngle(0, ms.value()).normalize())
            },
            Angle::ArcSeconds(s) => {
                AngleArcDegreesMinutes(ShortAngle(0, s.0 / 60.0).normalize())
            },
            Angle::TimeHours(h) => {
                AngleArcDegreesMinutes((15.0 * h.0).into())
            },
            Angle::TimeHoursMinutes(hm) => {
                AngleArcDegreesMinutes((15.0 * hm.value()).into())
            },
            Angle::TimeHoursMinutesSeconds(hms) => {
                AngleArcDegreesMinutes((15.0 * hms.value()).into())
            }
            Angle::TimeMinutes(m) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * m.0).normalize())
            },
            Angle::TimeMinutesSeconds(ms) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * ms.value()).normalize())
            },
            Angle::TimeSeconds(s) => {
                AngleArcDegreesMinutes(ShortAngle(0, s.0 / 4.0).normalize())
            }
        }
    }
}

impl convert::Into<Option<AngleArcDegreesMinutes>> for Angle {
    fn into(self) -> Option<AngleArcDegreesMinutes> {
        match self {
            Angle::ArcDegreesMinutes(dm) => Some(dm),
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

    pub fn is_adm(&self) -> bool {
        match self {
            Angle::ArcDegreesMinutes(_) => true,
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

    pub fn to_adm(self) -> Angle {
        Angle::ArcDegreesMinutes(self.into())
    }
}