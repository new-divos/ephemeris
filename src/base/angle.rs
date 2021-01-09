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
            Angle::ArcDegrees(AngleArcDegrees(d)) => {
                AngleRevolutions(d / 360.0)
            },
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(dm)) => {
                AngleRevolutions(dm.value() / 360.0)
            },
            Angle::ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds(dms)) => {
                AngleRevolutions(dms.value() / 360.0)
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleRevolutions(m / 21600.0)
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                AngleRevolutions(ms.value() / 21600.0)
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleRevolutions(s / 1296000.0)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleRevolutions(h / 24.0)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleRevolutions(hm.value() / 24.0)
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleRevolutions(hms.value() / 24.0)
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleRevolutions(m / 1440.0)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleRevolutions(ms.value() / 1440.0)
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleRevolutions(s / 86400.0)
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
            Angle::Revolutions(AngleRevolutions(r)) => {
                AngleArcDegrees(360.0 * r)
            },
            Angle::ArcDegrees(d) => d,
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(dm)) => {
                AngleArcDegrees(dm.value())
            },
            Angle::ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds(dms)) => {
                AngleArcDegrees(dms.value())
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleArcDegrees(m / 60.0)
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                AngleArcDegrees(ms.value() / 60.0)
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegrees(s / 3600.0)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegrees(h * 15.0)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcDegrees(hm.value() * 15.0)
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcDegrees(hms.value() * 15.0)
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegrees(m / 4.0)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcDegrees(ms.value() / 4.0)
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleArcDegrees(s / 240.0)
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
            Angle::Revolutions(AngleRevolutions(r)) => {
                AngleArcDegreesMinutes((360.0 * r).into())
            },
            Angle::ArcDegrees(AngleArcDegrees(d)) => {
                AngleArcDegreesMinutes(d.into())
            },
            Angle::ArcDegreesMinutes(dm) => dm,
            Angle::ArcDegreesMinutesSeconds(
                AngleArcDegreesMinutesSeconds(LongAngle(d, m, s))
            ) => {
                let ms = ShortAngle(m as i32, s);
                AngleArcDegreesMinutes(ShortAngle(d, ms.value()))
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleArcDegreesMinutes(ShortAngle(0, m).normalize())
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                AngleArcDegreesMinutes(ShortAngle(0, ms.value()).normalize())
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegreesMinutes(ShortAngle(0, s / 60.0).normalize())
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegreesMinutes((15.0 * h).into())
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcDegreesMinutes((15.0 * hm.value()).into())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcDegreesMinutes((15.0 * hms.value()).into())
            }
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * m).normalize())
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * ms.value()).normalize())
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleArcDegreesMinutes(ShortAngle(0, s / 4.0).normalize())
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

impl convert::Into<AngleArcDegreesMinutesSeconds> for Angle {
    fn into(self) -> AngleArcDegreesMinutesSeconds {
        match self {
            Angle::Radians(r) => {
                AngleArcDegreesMinutesSeconds((DEG * r).into())
            },
            Angle::Revolutions(AngleRevolutions(r)) => {
                AngleArcDegreesMinutesSeconds((360.0 * r).into())
            }
            Angle::ArcDegrees(AngleArcDegrees(d)) => {
                AngleArcDegreesMinutesSeconds(d.into())
            },
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(ShortAngle(d, m))) => {
                let ShortAngle(m, s) = m.into();
                AngleArcDegreesMinutesSeconds(LongAngle(d, m as i8, s))
            },
            Angle::ArcDegreesMinutesSeconds(dms) => dms,
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleArcDegreesMinutesSeconds((m / 60.0).into())
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                AngleArcDegreesMinutesSeconds((ms.value() / 60.0).into())
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegreesMinutesSeconds((s / 3600.0).into())
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegreesMinutesSeconds((15.0 * h).into())
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcDegreesMinutesSeconds((15.0 * hm.value()).into())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcDegreesMinutesSeconds((15.0 * hms.value()).into())
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegreesMinutesSeconds((m / 4.0).into())
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcDegreesMinutesSeconds((ms.value() / 4.0).into())
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleArcDegreesMinutesSeconds((s / 240.0).into())
            }
        }
    }
}

impl convert::Into<Option<AngleArcDegreesMinutesSeconds>> for Angle {
    fn into(self) -> Option<AngleArcDegreesMinutesSeconds> {
        match self {
            Angle::ArcDegreesMinutesSeconds(dms) => Some(dms),
            _ => None
        }
    }
}

impl convert::Into<AngleArcMinutes> for Angle {
    fn into(self) -> AngleArcMinutes {
        todo!();
    }
}

impl convert::Into<Option<AngleArcMinutes>> for Angle {
    fn into(self) -> Option<AngleArcMinutes> {
        match self {
            Angle::ArcMinutes(m) => Some(m),
            _ => None
        }
    }
}

impl convert::Into<AngleArcMinutesSeconds> for Angle {
    fn into(self) -> AngleArcMinutesSeconds {
        todo!();
    }
}

impl convert::Into<Option<AngleArcMinutesSeconds>> for Angle {
    fn into(self) -> Option<AngleArcMinutesSeconds> {
        match self {
            Angle::ArcMinutesSeconds(ms) => Some(ms),
            _ => None
        }
    }
}

impl convert::Into<AngleArcSeconds> for Angle {
    fn into(self) -> AngleArcSeconds {
        todo!();
    }
}

impl convert::Into<Option<AngleArcSeconds>> for Angle {
    fn into(self) -> Option<AngleArcSeconds> {
        match self {
            Angle::ArcSeconds(s) => Some(s),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeHours> for Angle {
    fn into(self) -> AngleTimeHours {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeHours>> for Angle {
    fn into(self) -> Option<AngleTimeHours> {
        match self {
            Angle::TimeHours(h) => Some(h),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeHoursMinutes> for Angle {
    fn into(self) -> AngleTimeHoursMinutes {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeHoursMinutes>> for Angle {
    fn into(self) -> Option<AngleTimeHoursMinutes> {
        match self {
            Angle::TimeHoursMinutes(tm) => Some(tm),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeHoursMinutesSeconds> for Angle {
    fn into(self) -> AngleTimeHoursMinutesSeconds {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeHoursMinutesSeconds>> for Angle {
    fn into(self) -> Option<AngleTimeHoursMinutesSeconds> {
        match self {
            Angle::TimeHoursMinutesSeconds(hms) => Some(hms),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeMinutes> for Angle {
    fn into(self) -> AngleTimeMinutes {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeMinutes>> for Angle {
    fn into(self) -> Option<AngleTimeMinutes> {
        match self {
            Angle::TimeMinutes(m) => Some(m),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeMinutesSeconds> for Angle {
    fn into(self) -> AngleTimeMinutesSeconds {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeMinutesSeconds>> for Angle {
    fn into(self) -> Option<AngleTimeMinutesSeconds> {
        match self {
            Angle::TimeMinutesSeconds(tm) => Some(tm),
            _ => None
        }
    }
}

impl convert::Into<AngleTimeSeconds> for Angle {
    fn into(self) -> AngleTimeSeconds {
        todo!();
    }
}

impl convert::Into<Option<AngleTimeSeconds>> for Angle {
    fn into(self) -> Option<AngleTimeSeconds> {
        match self {
            Angle::TimeSeconds(s) => Some(s),
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

    pub fn is_adms(&self) -> bool {
        match self {
            Angle::ArcDegreesMinutesSeconds(_) => true,
            _ => false
        }
    }

    pub fn is_am(&self) -> bool {
        match self {
            Angle::ArcMinutes(_) => true,
            _ => false
        }
    }

    pub fn is_ams(&self) -> bool {
        match self {
            Angle::ArcMinutesSeconds(_) => true,
            _ => false
        }
    }

    pub fn is_as(&self) -> bool {
        match self {
            Angle::ArcSeconds(_) => true,
            _ => false
        }
    }

    pub fn is_th(&self) -> bool {
        match self {
            Angle::TimeHours(_) => true,
            _ => false
        }
    }

    pub fn is_thm(&self) -> bool {
        match self {
            Angle::TimeHoursMinutes(_) => true,
            _ => false
        }
    }

    pub fn is_thms(&self) -> bool {
        match self {
            Angle::TimeHoursMinutesSeconds(_) => true,
            _ => false
        }
    }

    pub fn is_tm(&self) -> bool {
        match self {
            Angle::TimeMinutes(_) => true,
            _ => false
        }
    }

    pub fn is_tms(&self) -> bool {
        match self {
            Angle::TimeMinutesSeconds(_) => true,
            _ => false
        }
    }

    pub fn is_ts(&self) -> bool {
        match self {
            Angle::TimeSeconds(_) => true,
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

    pub fn to_adms(self) -> Angle {
        Angle::ArcDegreesMinutesSeconds(self.into())
    }

    pub fn to_am(self) -> Angle {
        Angle::ArcMinutes(self.into())
    }

    pub fn to_ams(self) -> Angle {
        Angle::ArcMinutesSeconds(self.into())
    }

    pub fn to_as(self) -> Angle {
        Angle::ArcSeconds(self.into())
    }

    pub fn to_th(self) -> Angle {
        Angle::TimeHours(self.into())
    }

    pub fn to_thm(self) -> Angle {
        Angle::TimeHoursMinutes(self.into())
    }

    pub fn to_thms(self) -> Angle {
        Angle::TimeHoursMinutesSeconds(self.into())
    }

    pub fn to_tm(self) -> Angle {
        Angle::TimeMinutes(self.into())
    }

    pub fn to_tms(self) -> Angle {
        Angle::TimeMinutesSeconds(self.into())
    }

    pub fn to_ts(self) -> Angle {
        Angle::TimeSeconds(self.into())
    }
}