use std::convert;
use std::default;

use crate::base::consts::{ARCS, DEG, MULT_2_PI, RAD};

const TSEC: f64 = ARCS / 15.0;

const RAD_IN_AMIN: f64 = RAD / 60.0;
const RAD_IN_THRS: f64 = 15.0 * RAD;
const RAD_IN_TMIN: f64 = RAD / 4.0;

const ADEG_IN_REV: f64 = 360.0;
const AMIN_IN_REV: f64 = ADEG_IN_REV * 60.0;
const ASEC_IN_REV: f64 = AMIN_IN_REV * 60.0;

const THRS_IN_REV: f64 = 24.0;
const TMIN_IN_REV: f64 = THRS_IN_REV * 60.0;
const TSEC_IN_REV: f64 = TMIN_IN_REV * 60.0;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}

trait AngleSign {
    fn sign(&self) -> Sign;
}

trait AngleTransform {
    fn copysign(&self, value: f64) -> Self;
    fn normalize(&self) -> Self;
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Left(f64);  // Left value.

#[derive(Debug, Copy, Clone, PartialEq)]
struct Middle(f64);  // Middle value.

#[derive(Debug, Copy, Clone, PartialEq)]
struct Right(f64);  // Right value.


#[derive(Debug, Copy, Clone, PartialEq)]
struct ShortAngle(i32, f64);

impl default::Default for ShortAngle {
    fn default() -> Self {
        Self(0, 0.0)
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

impl convert::Into<ShortAngle> for Left {
    fn into(self) -> ShortAngle {
        let v = self.0.abs();
        let u = v.floor();

        let result = ShortAngle(u as i32, 60.0 * (v - u));
        result.copysign(self.0)
    }
}

impl convert::Into<Left> for ShortAngle {
    fn into(self) -> Left {
        let mut value = (self.0.abs() as f64) + self.1.abs() / 60.0;
        if self.0 < 0 || (self.0 == 0 && self.1 < 0.0) {
            value = -value
        }

        Left(value)
    }
}

impl convert::Into<ShortAngle> for Right {
    fn into(self) -> ShortAngle {
        let v = self.0.abs();
        let u = (v / 60.0).floor();

        let result = ShortAngle(u as i32, v - 60.0 * u);
        result.copysign(self.0)
    }
}

impl convert::Into<Right> for ShortAngle {
    fn into(self) -> Right {
        if self.0 != 0 {
            let mut value = 60.0 * (self.0.abs() as f64) + self.1;
            if self.0 < 0 { value = -value }
            Right(value)
        } else {
            Right(self.1)
        }
    }
}

impl AngleTransform for ShortAngle {
    fn copysign(&self, value: f64) -> Self {
        let ShortAngle(value1, value2) = *self;

        if value1 != 0 {
            let mut value1 = value1.abs();
            if value < 0.0 { value1 = -value1 }
            ShortAngle(value1, value2)
        } else {
            ShortAngle(value1, value2.copysign(value))
        }
    }

    fn normalize(&self) -> Self {
        let t = 60.0 * (self.0 as f64) + self.1;
        let v = t.abs();
        let u = (t / 60.0).floor();

        let result = ShortAngle(u as i32, v - 60.0 * u);
        result.copysign(t)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct LongAngle(i32, i8, f64);

impl default::Default for LongAngle {
    fn default() -> Self {
        Self(0, 0, 0.0)
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

impl convert::Into<LongAngle> for Left {
    fn into(self) -> LongAngle {
        let v = self.0.abs();
        let u = v.floor();
        let w = 60.0 * (v - u);
        let m = (w / 60.0).floor();

        let result = LongAngle(u as i32, m as i8, 60.0 * (w - m));
        result.copysign(self.0)
    }
}

impl convert::Into<Left> for LongAngle {
    fn into(self) -> Left {
        let mut value = (self.0.abs() as f64) + ((self.1.abs() as f64) + self.2.abs() / 60.0) / 60.0;
        if self.0 < 0 || (self.0 == 0 &&
            (self.1 < 0 || (self.1 == 0 && self.2 < 0.0))
        ) { value = -value }

        Left(value)
    }
}

impl convert::Into<LongAngle> for Middle {
    fn into(self) -> LongAngle {
        let v = self.0.abs();
        let w = v.floor();
        let u = (w / 60.0).floor();
        let m = w - 60.0 * u;

        let result = LongAngle(u as i32, m as i8, 60.0 * (v - w));
        result.copysign(self.0)
    }
}

impl convert::Into<Middle> for LongAngle {
    fn into(self) -> Middle {
        if self.0 != 0 {
            let mut value =
                60.0 * (self.0.abs() as f64) + (self.1.abs() as f64) + self.2.abs() / 60.0;
            if self.0 < 0 { value = -value }
            Middle(value)
        } else if self.1 != 0 {
            let mut value = (self.1.abs() as f64) + self.2.abs() / 60.0;
            if self.1 < 0 { value = -value }
            Middle(value)
        } else {
            Middle(self.2 / 60.0)
        }
    }
}

impl convert::Into<LongAngle> for Right {
    fn into(self) -> LongAngle {
        let v = self.0.abs();
        let w = (v / 60.0).floor();
        let u = (w / 60.0).floor();
        let m = w - 60.0 * u;

        let result = LongAngle(u as i32, m as i8, v - 60.0 * w);
        result.copysign(self.0)
    }
}

impl convert::Into<Right> for LongAngle {
    fn into(self) -> Right {
        if self.0 != 0 {
            let mut value =
                self.2.abs() + 60.0 * ((self.1.abs() as f64) + 60.0 * (self.0.abs() as f64));
            if self.0 < 0 { value = -value }
            Right(value)
        } else if self.1 != 0 {
            let mut value = self.2.abs() + 60.0 * (self.1.abs() as f64);
            if self.1 < 0 { value = -value }
            Right(value)
        } else {
            Right(self.2)
        }
    }
}

impl AngleTransform for LongAngle {
    fn copysign(&self, value: f64) -> Self {
        let LongAngle(value1, value2, value3) = *self;

        if value1 != 0 {
            let mut value1 = value1.abs();
            if value < 0.0 { value1 = -value1 };
            LongAngle(value1, value2, value3)
        } else if value2 != 0 {
            let mut value2 = value2.abs();
            if value < 0.0 { value2 = -value2 }
            LongAngle(value1, value2, value3)
        } else {
            LongAngle(value1, value2, value3.copysign(value))
        }
    }

    fn normalize(&self) -> Self {
        let t = 60.0 * ((self.0 * 60 + self.1 as i32) as f64) + self.2;
        let v = t.abs();
        let w = (v / 60.0).floor();
        let u = (w / 60.0).floor();
        let m = w - 60.0 * u;

        let result = LongAngle(u as i32, m as i8, v - 60.0 * w);
        result.copysign(t)
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
        self.value() * RAD
    }
}

impl AngleArcDegreesMinutes {
    pub fn degrees(&self) -> i32 {
        let AngleArcDegreesMinutes(ShortAngle(degrees, _)) = *self;
        degrees.abs()
    }

    pub fn minutes(&self) -> f64 {
        let AngleArcDegreesMinutes(ShortAngle(_, minutes)) = *self;
        minutes.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.value() * RAD
    }
}

impl AngleArcDegreesMinutesSeconds {
    pub fn degrees(&self) -> i32 {
        let AngleArcDegreesMinutesSeconds(LongAngle(degrees, ..)) = *self;
        degrees.abs()
    }

    pub fn minutes(&self) -> i32 {
        let AngleArcDegreesMinutesSeconds(LongAngle(_, minutes, _)) = *self;
        minutes.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        let AngleArcDegreesMinutesSeconds(LongAngle(.., seconds)) = *self;
        seconds.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.0 * RAD_IN_AMIN
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
        self.value() * RAD_IN_AMIN
    }
}

impl AngleArcMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        let AngleArcMinutesSeconds(ShortAngle(minutes, _)) = *self;
        minutes.abs()
    }

    pub fn seconds(&self) -> f64 {
        let AngleArcMinutesSeconds(ShortAngle(_, seconds)) = *self;
        seconds.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.0 * RAD_IN_THRS
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
        self.value() * RAD_IN_THRS
    }
}

impl AngleTimeHoursMinutes {
    pub fn hours(&self) -> i32 {
        let AngleTimeHoursMinutes(ShortAngle(hours, _)) = *self;
        hours.abs()
    }

    pub fn minutes(&self) -> f64 {
        let AngleTimeHoursMinutes(ShortAngle(_, minutes)) = *self;
        minutes.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.value() * RAD_IN_THRS
    }
}

impl AngleTimeHoursMinutesSeconds {
    pub fn hours(&self) -> i32 {
        let AngleTimeHoursMinutesSeconds(LongAngle(hours, ..)) = *self;
        hours.abs()
    }

    pub fn minutes(&self) -> i32 {
        let AngleTimeHoursMinutesSeconds(LongAngle(_, minutes, _)) = *self;
        minutes.abs() as i32
    }

    pub fn seconds(&self) -> f64 {
        let AngleTimeHoursMinutesSeconds(LongAngle(.., seconds)) = *self;
        seconds.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.0 * RAD_IN_TMIN
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
        self.value() * RAD_IN_TMIN
    }
}

impl AngleTimeMinutesSeconds {
    pub fn minutes(&self) -> i32 {
        let AngleTimeMinutesSeconds(ShortAngle(minutes, _)) = *self;
        minutes.abs()
    }

    pub fn seconds(&self) -> f64 {
        let AngleTimeMinutesSeconds(ShortAngle(_, seconds)) = *self;
        seconds.abs()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn value(&self) -> f64 {
        let Left(value) = self.0.into();
        value
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
        self.0 / TSEC
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
                AngleRevolutions(d / ADEG_IN_REV)
            },
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(dm)) => {
                let Left(degrees) = dm.into();
                AngleRevolutions(degrees / ADEG_IN_REV)
            },
            Angle::ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds(dms)) => {
                let Left(degrees) = dms.into();
                AngleRevolutions(degrees / ADEG_IN_REV)
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleRevolutions(m / AMIN_IN_REV)
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                let Left(minutes) = ms.into();
                AngleRevolutions(minutes / AMIN_IN_REV)
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleRevolutions(s / ASEC_IN_REV)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleRevolutions(h / THRS_IN_REV)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                let Left(hours) = hm.into();
                AngleRevolutions(hours / THRS_IN_REV)
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                let Left(hours) = hms.into();
                AngleRevolutions(hours / THRS_IN_REV)
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleRevolutions(m / TMIN_IN_REV)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                let Left(minutes) = ms.into();
                AngleRevolutions(minutes / TMIN_IN_REV)
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleRevolutions(s / TSEC_IN_REV)
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
                AngleArcDegrees(ADEG_IN_REV * r)
            },
            Angle::ArcDegrees(d) => d,
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(dm)) => {
                let Left(degrees) = dm.into();
                AngleArcDegrees(degrees)
            },
            Angle::ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds(dms)) => {
                let Left(degrees) = dms.into();
                AngleArcDegrees(degrees)
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleArcDegrees(m / 60.0)
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                let Left(minutes) = ms.into();
                AngleArcDegrees(minutes / 60.0)
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegrees(s / 3600.0)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegrees(h * 15.0)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                let Left(hours) = hm.into();
                AngleArcDegrees(hours * 15.0)
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                let Left(hours) = hms.into();
                AngleArcDegrees(hours * 15.0)
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegrees(m / 4.0)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                let Left(minutes) = ms.into();
                AngleArcDegrees(minutes / 4.0)
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
                AngleArcDegreesMinutes(ShortAngle::from_left_value(DEG * r))
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
                AngleArcDegreesMinutes(ShortAngle(d, ms.left_value()))
            },
            Angle::ArcMinutes(AngleArcMinutes(m)) => {
                AngleArcDegreesMinutes(ShortAngle(0, m).normalize())
            },
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ms)) => {
                AngleArcDegreesMinutes(ShortAngle(0, ms.left_value()).normalize())
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegreesMinutes(ShortAngle(0, s / 60.0).normalize())
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegreesMinutes((15.0 * h).into())
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcDegreesMinutes((15.0 * hm.left_value()).into())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcDegreesMinutes((15.0 * hms.left_value()).into())
            }
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * m).normalize())
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcDegreesMinutes(ShortAngle(0, 15.0 * ms.left_value()).normalize())
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
                AngleArcDegreesMinutesSeconds((ms.left_value() / 60.0).into())
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcDegreesMinutesSeconds((s / 3600.0).into())
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcDegreesMinutesSeconds((15.0 * h).into())
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcDegreesMinutesSeconds((15.0 * hm.left_value()).into())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcDegreesMinutesSeconds((15.0 * hms.left_value()).into())
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcDegreesMinutesSeconds((m / 4.0).into())
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcDegreesMinutesSeconds((ms.left_value() / 4.0).into())
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
        match self {
            Angle::Radians(r) => {
                AngleArcMinutes(60.0 * DEG * r)
            },
            Angle::Revolutions(AngleRevolutions(r)) => {
                AngleArcMinutes(21600.0 * r)
            },
            Angle::ArcDegrees(AngleArcDegrees(d)) => {
                AngleArcMinutes(60.0 * d)
            },
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(ShortAngle(d, m))) => {
                if d != 0 {
                    let mut m = 60.0 * (d.abs() as f64) + m;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else {
                    AngleArcMinutes(m)
                }
            },
            Angle::ArcDegreesMinutesSeconds(
                AngleArcDegreesMinutesSeconds(LongAngle(d, m, s))
            ) => {
                if d != 0 {
                    let mut m = 60.0 * (d.abs() as f64) + (m as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else if m != 0 {
                    let d = m;
                    let mut m = (m.abs() as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else {
                    AngleArcMinutes(s / 60.0)
                }
            },
            Angle::ArcMinutes(m) => m,
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ShortAngle(m, s))) => {
                if m != 0 {
                    let d = m;
                    let mut m = (m.abs() as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else {
                    AngleArcMinutes(s / 60.0)
                }
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcMinutes(s / 60.0)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcMinutes(900.0 * h)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcMinutes(900.0 * hm.left_value())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcMinutes(900.0 * hms.left_value())
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcMinutes(15.0 * m)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcMinutes(15.0 * ms.left_value())
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleArcMinutes(s / 60.0)
            }
        }
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
        match self {
            Angle::Radians(r) => {
                AngleArcMinutesSeconds((60.0 * DEG * r).into())
            },
            Angle::Revolutions(AngleRevolutions(r)) => {
                AngleArcMinutesSeconds((21600.0 * r).into())
            },
            Angle::ArcDegrees(AngleArcDegrees(d)) => {
                AngleArcMinutesSeconds((60.0 * d).into())
            },
            Angle::ArcDegreesMinutes(AngleArcDegreesMinutes(ShortAngle(d, m))) => {
                let m = if d != 0 {
                    let v = 60.0 * (d.abs() as f64) + m;
                    if d < 0 { -v } else { v }
                } else {
                    m
                };

                AngleArcMinutesSeconds(m.into())
            },
            Angle::ArcDegreesMinutesSeconds(
                AngleArcDegreesMinutesSeconds(LongAngle(d, m, s))
            ) => {
                if d != 0 {
                    let mut m = 60.0 * (d.abs() as f64) + (m as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else if m != 0 {
                    let d = m;
                    let mut m = (m.abs() as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else {
                    AngleArcMinutes(s / 60.0)
                }
            },
            Angle::ArcMinutes(m) => m,
            Angle::ArcMinutesSeconds(AngleArcMinutesSeconds(ShortAngle(m, s))) => {
                if m != 0 {
                    let d = m;
                    let mut m = (m.abs() as f64) + s / 60.0;
                    if d < 0 { m = -m }
                    AngleArcMinutes(m)
                } else {
                    AngleArcMinutes(s / 60.0)
                }
            },
            Angle::ArcSeconds(AngleArcSeconds(s)) => {
                AngleArcMinutes(s / 60.0)
            },
            Angle::TimeHours(AngleTimeHours(h)) => {
                AngleArcMinutes(900.0 * h)
            },
            Angle::TimeHoursMinutes(AngleTimeHoursMinutes(hm)) => {
                AngleArcMinutes(900.0 * hm.left_value())
            },
            Angle::TimeHoursMinutesSeconds(AngleTimeHoursMinutesSeconds(hms)) => {
                AngleArcMinutes(900.0 * hms.left_value())
            },
            Angle::TimeMinutes(AngleTimeMinutes(m)) => {
                AngleArcMinutes(15.0 * m)
            },
            Angle::TimeMinutesSeconds(AngleTimeMinutesSeconds(ms)) => {
                AngleArcMinutes(15.0 * ms.left_value())
            },
            Angle::TimeSeconds(AngleTimeSeconds(s)) => {
                AngleArcMinutes(s / 60.0)
            }
        }
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