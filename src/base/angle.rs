use std::convert;
use std::default;

use crate::base::consts::{MULT_2_PI, RAD};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleArcDegreesMinutes {
    degrees: i64,
    minutes: f64
}

impl default::Default for AngleArcDegreesMinutes {
    fn default() -> Self {
        Self {
            degrees: 0,
            minutes: 0.0
        }
    }
}

impl convert::From<f64> for AngleArcDegreesMinutes {
    fn from(degrees: f64) -> Self {
        let d = degrees.trunc();
        let mut minutes = 60.0 * (degrees - d);

        let d = d as i64;
        if d != 0 && minutes < 0.0 {
            minutes = -minutes;
        }

        Self {
            degrees: d,
            minutes
        }
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutes {
    fn into(self) -> f64 {
        let mut result = (self.degrees() as f64) + self.minutes() / 60.0;
        if self.sign() < 0 {
            result = -result;
        }

        result
    }
}

impl AngleArcDegreesMinutes {
    pub fn degrees(&self) -> i64 {
        self.degrees.abs()
    }

    pub fn minutes(&self) -> f64 {
        self.minutes.abs()
    }

    pub fn sign(&self) -> i32 {
        if self.degrees == 0 && self.minutes == 0.0 {
            0
        } else if self.degrees < 0 || (self.degrees == 0 && self.minutes < 0.0) {
            -1
        } else {
            1
        }
    }

    fn new(degrees: i64, minutes: f64) -> AngleArcDegreesMinutes {
        let total = 60.0 * (degrees as f64) + minutes;

        let t = total.abs();
        let mut degrees = (t / 60.0).floor();
        let mut minutes = t - 60.0 * degrees;

        if degrees == 0.0 {
            minutes = minutes.copysign(total);
        } else {
            degrees = degrees.copysign(total);
        }

        AngleArcDegreesMinutes {
            degrees: degrees as i64,
            minutes
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AngleArcDegreesMinutesSeconds {
    degrees: i64,
    minutes: i8,
    seconds: f64
}

impl default::Default for AngleArcDegreesMinutesSeconds {
    fn default() -> Self {
        Self {
            degrees: 0,
            minutes: 0i8,
            seconds: 0.0
        }
    }
}

impl convert::From<f64> for AngleArcDegreesMinutesSeconds {
    fn from(degrees: f64) -> Self {
        let d = degrees.trunc();
        let minutes = 60.0 * (degrees - d);
        let m = minutes.trunc();
        let mut seconds = 60.0 * (minutes - m);

        let d = d as i64;
        let mut m = m as i8;
        if d != 0 {
            if m < 0 {
                m = -m;
            }
            if seconds < 0.0 {
                seconds = -seconds;
            }
        } else if m != 0 && seconds < 0.0 {
            seconds = -seconds;
        }

        Self {
            degrees: d,
            minutes: m,
            seconds
        }
    }
}

impl convert::Into<f64> for AngleArcDegreesMinutesSeconds {
    fn into(self) -> f64 {
         let result = (self.degrees() as f64) + ((self.minutes() as f64)
            + self.seconds() / 60.0) / 60.0;
        if self.sign() < 0 {
            result = -result;
        }

        result
    }
}

impl AngleArcDegreesMinutesSeconds {
    pub fn degrees(&self) -> i64 {
        self.degrees.abs()
    }

    pub fn minutes(&self) -> i8 {
        self.minutes.abs()
    }

    pub fn seconds(&self) -> f64 {
        self.seconds.abs()
    }

    pub fn sign(&self) -> i32 {
        if self.degrees == 0 && self.minutes == 0 && self.seconds == 0.0 {
            0
        } else if self.degrees < 0 || (
            self.degrees == 0 && (
                self.minutes < 0 || (self.minutes == 0 && self.seconds < 0.0)
            )
        ) {
            -1
        } else {
            1
        }
    }

    fn new(degrees: i64, minutes: i64, seconds: f64) -> AngleArcDegreesMinutesSeconds {
        let total = 60.0 * ((degrees * 60 + minutes) as f64)
            + seconds;

        let t = total.abs();
        let minutes = (t / 60.0).floor();
        let mut seconds = t - 60.0 * minutes;
        let mut degrees = (minutes / 60.0).floor();
        let mut minutes = minutes - 60.0 * degrees;

        if degrees == 0.0 {
            if minutes == 0.0 {
                seconds = seconds.copysign(total);
            } else {
                minutes = minutes.copysign(total);
            }
        } else {
            degrees = degrees.copysign(total);
        }

        AngleArcDegreesMinutesSeconds {
            degrees: degrees as i64,
            minutes: minutes as i8,
            seconds
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f64),
    Revolutions(f64),
    ArcDegrees(f64),
    ArcDegreesMinutes(AngleArcDegreesMinutes),
    ArcDegreesMinutesSeconds(AngleArcDegreesMinutesSeconds),
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
            Angle::ArcDegreesMinutes(ref dm) => {
                let d: f64 = dm.into();
                RAD * d
            },
            Angle::ArcDegreesMinutesSeconds(ref dms) => {
                let d: f64 = dms.into();
                RAD * d
            }
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

    pub fn from_adm(degrees: i64, minutes: f64) -> Angle {
        Angle::ArcDegreesMinutes(AngleArcDegreesMinutes::new(degrees, minutes))
    }

    pub fn from_adms(degrees: i64, minutes: i64, seconds: f64) -> Angle {
        Angle::ArcDegreesMinutesSeconds(
            AngleArcDegreesMinutesSeconds::new(degrees, minutes, seconds)
        )
    }
}