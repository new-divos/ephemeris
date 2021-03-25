use std::cmp::Ordering;
use std::convert;
use std::marker::PhantomData;

use crate::base::consts::{PI2, D2R, R2D, R2AM, AM2R, R2AS, AS2R,
                          H2R, R2H, M2R, R2M, S2R, R2S};


const RV2D: f64 = 360.0;
const RV2AM: f64 = RV2D * 60.0;
const RV2AS: f64 = RV2AM * 60.0;

const RV2H: f64 = 24.0;
const RV2M: f64 = RV2H * 60.0;
const RV2S: f64 = RV2M * 60.0;

const TA: f64 = 15.0;
const TM: f64 = 60.0 / TA;
const TS: f64 = 3600.0 / TA;

const TMM: f64 = TA * 60.0;
const TMS: f64 = TMM * 60.0;


///
/// Left value in a composed angle value
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Left(f64);

///
/// Middle value in a composed angle value
///
#[derive(Copy, Clone, Debug, PartialEq)]
struct Middle(f64);

///
/// Right value in a composed angle value
///
#[derive(Copy, Clone, Debug, PartialEq)]
struct Right(f64);


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}


#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ShortAngle(i32, f64);

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
        if Self::is_negative(self.0, self.1) {
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

impl convert::Into<(i32, f64)> for ShortAngle {
    fn into(self) -> (i32, f64) {
        let ShortAngle(value1, value2) = self;
        (value1, value2)
    }
}

impl convert::Into<(Sign, i32, f64)> for ShortAngle {
    fn into(self) -> (Sign, i32, f64) {
        let ShortAngle(value1, value2) = self;
        (self.sign(), value1.abs(), value2.abs())
    }
}

impl PartialOrd for ShortAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1.is_nan() || other.1.is_nan() {
            None
        } else {
            let Left(value1) = (*self).into();
            let Left(value2) = (*other).into();

            value1.partial_cmp(&value2)
        }
    }
}

impl ShortAngle {
    fn copysign(&self, value: f64) -> Self {
        let ShortAngle(value1, value2) = *self;

        let mut value1 = value1.abs();
        let mut value2 = value2.abs();

        if value < 0.0 {
            if value1 == 0 {
                value2 = -value2;
            } else {
                value1 = -value1;
            }
        }

        ShortAngle(value1, value2)
    }

    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0.0 {
            Sign::Zero
        } else if Self::is_negative(self.0, self.1) {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    #[inline]
    fn is_negative(value1: i32, value2: f64) -> bool {
        value1 < 0 || (value1 == 0 && value2 < 0.0)
    }

    #[inline]
    fn new(value1: i32, value2: f64) -> Self {
        Right(
            if Self::is_negative(value1, value2) {
                -((value1.abs() as f64) * 60.0 + value2.abs())
            } else {
                (value1 as f64) * 60.0 + value2
            }
        ).into()
    }
}


#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LongAngle(i32, i8, f64);

impl convert::Into<LongAngle> for Left {
    fn into(self) -> LongAngle {
        let v = self.0.abs();
        let u = v.floor();
        let w = 60.0 * (v - u);
        let m = w.floor();

        let result = LongAngle(u as i32, m as i8, 60.0 * (w - m));
        result.copysign(self.0)
    }
}

impl convert::Into<Left> for LongAngle {
    fn into(self) -> Left {
        let mut value = (self.0.abs() as f64) +
            ((self.1.abs() as f64) + self.2.abs() / 60.0) / 60.0;
        if Self::is_negative(self.0, self.1 as i32, self.2) {
            value = -value
        }

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
                self.2 + 60.0 * ((self.1 as f64) + 60.0 * (self.0.abs() as f64));
            if self.0 < 0 {
                value = -value
            }
            Right(value)
        } else if self.1 != 0 {
            let mut value = self.2 + 60.0 * (self.1.abs() as f64);
            if self.1 < 0 { value = -value }
            Right(value)
        } else {
            Right(self.2)
        }
    }
}

impl convert::Into<(i32, i32, f64)> for LongAngle {
    fn into(self) -> (i32, i32, f64) {
        let LongAngle(value1, value2, value3) = self;
        (value1, value2 as i32, value3)
    }
}

impl convert::Into<(Sign, i32, i32, f64)> for LongAngle {
    fn into(self) -> (Sign, i32, i32, f64) {
        let LongAngle(value1, value2, value3) = self;
        (self.sign(), value1.abs(), value2.abs() as i32, value3.abs())
    }
}

impl PartialOrd for LongAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.2.is_nan() || other.2.is_nan() {
            None
        } else {
            let Left(value1) = (*self).into();
            let Left(value2) = (*other).into();

            value1.partial_cmp(&value2)
        }
    }
}

impl LongAngle {
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

    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0 && self.2 == 0.0 {
            Sign::Zero
        } else if Self::is_negative(self.0, self.1 as i32, self.2) {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    #[inline]
    fn is_negative(value1: i32, value2: i32, value3: f64) -> bool {
        value1 < 0 || (value1 == 0 && (value2 < 0 ||(value2 == 0 && value3 < 0.0)))
    }

    #[inline]
    fn new(value1: i32, value2: i32, value3: f64) -> Self {
        Right(
            if Self::is_negative(value1, value2, value3) {
                -(((value1.abs() * 60 + value2.abs()) as f64) * 60.0 + value3.abs())
            } else {
                ((value1 * 60 + value2) as f64) * 60.0 + value3
            }
        ).into()
    }
}


pub trait AngleMapper {
    type Item: Copy;
}

#[derive(Clone, Copy)]
pub struct Angle<T: AngleMapper + Copy>
(
    <T as AngleMapper>::Item,
    PhantomData<T>
);


macro_rules! angle_new {
    ($t:ty; $v:ident) => {
        impl Angle<$t> {
            #[inline]
            pub fn $v(&self) -> f64 {
                self.0
            }

            pub fn sign(&self) -> Sign {
                if self.0 > 0.0 {
                    Sign::Positive
                } else if self.0 < 0.0 {
                    Sign::Negative
                } else {
                    Sign::Zero
                }
            }

            #[inline]
            pub fn new($v: f64) -> Self {
                Self($v, PhantomData::<$t>)
            }
        }
    };
    ($t:ty; $v1:ident, $v2:ident) => {
        impl Angle<$t> {
            #[inline]
            pub fn $v1(&self) -> i32 {
                self.0.0
            }

            #[inline]
            pub fn $v2(&self) -> f64 {
                self.0.1
            }

            #[inline]
            pub fn sign(&self) -> Sign {
                self.0.sign()
            }

            #[inline]
            pub fn new($v1: i32, $v2: f64) -> Self {
                Self(ShortAngle::new($v1, $v2), PhantomData::<$t>)
            }
        }
    };
    ($t:ty; $v1:ident, $v2:ident, $v3:ident) => {
        impl Angle<$t> {
            #[inline]
            pub fn $v1(&self) -> i32 {
                self.0.0
            }

            #[inline]
            pub fn $v2(&self) -> i32 {
                self.0.1 as i32
            }

            #[inline]
            pub fn $v3(&self) -> f64 {
                self.0.2
            }

            #[inline]
            pub fn sign(&self) -> Sign {
                self.0.sign()
            }

            #[inline]
            pub fn new($v1: i32, $v2: i32, $v3: f64) -> Self {
                Self(LongAngle::new($v1, $v2, $v3), PhantomData::<$t>)
            }
        }
    };
}


macro_rules! impl_into {
    () => {};
    ($td:ty: 0 = value * $c:expr; $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(value: f64) -> Self {
                Self(value * $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($td:ty: 0 = value / $c:expr; $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(value: f64) -> Self {
                Self(value / $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($td:ty: $m:ident = value * $c:expr; $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(value: f64) -> Self {
                Self(
                    $m(value * $c).into(),
                    PhantomData::<$td>
                )
            }
        }

        impl_into!($($tail)*);
    };
    ($td:ty: $m:ident = value * $c:expr; $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(value: f64) -> Self {
                Self(
                    $m(value / $c).into(),
                    PhantomData::<$td>
                )
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty: 0 * $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                self.0 * $c
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty: 0 / $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                self.0 / $c
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty : $m:ident * $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            fn into(self) -> f64 {
                let $m(value) = self.0.into();
                value * $c
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty: $m:ident / $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            fn into(self) -> f64 {
                let $m(value) = self.0.into();
                value / $c
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => Radians; $($tail:tt)*) => {
        impl convert::Into<Angle<Radians>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<Radians> {
                Angle::<Radians>(self.into(), PhantomData::<Radians>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: 0 = 0 * $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<$td> {
                Angle::<$td>(self.0 * $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: 0 = 0 / $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<$td> {
                Angle::<$td>(self.0 / $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $m:ident = 0; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<$td> {
                Angle::<$td>($m(self.0).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $m:ident = 0 * $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<$td> {
                Angle::<$td>($m(self.0 * $c).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $m:ident = 0 / $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            #[inline]
            fn into(self) -> Angle<$td> {
                Angle::<$td>($m(self.0 / $c).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: 0 = $m:ident; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $m(value) = self.0.into();
                Angle::<$td>(value, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: 0 = $m:ident * $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $m(value) = self.0.into();
                Angle::<$td>(value * $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: 0 = $m:ident / $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $m(value) = self.0.into();
                Angle::<$td>(value / $c, PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $md:ident = $ms:ident; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $ms(value) = self.0.into();
                Angle::<$td>($md(value).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $md:ident = $ms:ident * $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $ms(value) = self.0.into();
                Angle::<$td>($md(value * $c).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty => $td:ty: $md:ident = $ms:ident / $c:expr; $($tail:tt)*) => {
        impl convert::Into<Angle<$td>> for Angle<$ts> {
            fn into(self) -> Angle<$td> {
                let $ms(value) = self.0.into();
                Angle::<$td>($md(value / $c).into(), PhantomData::<$td>)
            }
        }

        impl_into!($($tail)*);
    };
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Radians;

impl convert::From<f64> for Angle<Radians> {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value, PhantomData::<Radians>)
    }
}

impl convert::Into<f64> for Angle<Radians> {
    #[inline]
    fn into(self) -> f64 {
        self.0
    }
}

angle_new!(Radians; radians);
impl_into! {
    Radians => Revolutions: 0 = 0 / PI2;
    Radians => Degrees: 0 = 0 * R2D;
    Radians => DegreesArcMinutes: Left = 0 * R2D;
    Radians => DegreesArcMinutesSeconds: Left = 0 * R2D;
    Radians => ArcMinutes: 0 = 0 * R2AM;
    Radians => ArcMinutesSeconds: Left = 0 * R2AM;
    Radians => ArcSeconds: 0 = 0 * R2AS;
    Radians => Hours: 0 = 0 * R2H;
    Radians => HoursMinutes: Left = 0 * R2H;
    Radians => HoursMinutesSeconds: Left = 0 * R2H;
    Radians => Minutes: 0 = 0 * R2M;
    Radians => MinutesSeconds: Left = 0 * R2M;
    Radians => Seconds: 0 = 0 * R2S;
}

angle_serialize!(Radians);
angle_deserialize!(Radians);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Revolutions;

angle_new!(Revolutions; revolutions);
impl_into! {
    Revolutions: 0 = value / PI2;
    Revolutions: 0 * PI2;
    Revolutions => Radians;
    Revolutions => Degrees: 0 = 0 * RV2D;
    Revolutions => DegreesArcMinutes: Left = 0 * RV2D;
    Revolutions => DegreesArcMinutesSeconds: Left = 0 * RV2D;
    Revolutions => ArcMinutes: 0 = 0 * RV2AM;
    Revolutions => ArcMinutesSeconds: Left = 0 * RV2AM;
    Revolutions => ArcSeconds: 0 = 0 * RV2AS;
    Revolutions => Hours: 0 = 0 * RV2H;
    Revolutions => HoursMinutes: Left = 0 * RV2H;
    Revolutions => HoursMinutesSeconds: Left = 0 * RV2H;
    Revolutions => Minutes: 0 = 0 * RV2M;
    Revolutions => MinutesSeconds: Left = 0 * RV2M;
    Revolutions => Seconds: 0 = 0 * RV2S;
}

angle_serialize!(Revolutions);
angle_deserialize!(Revolutions);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Degrees;

angle_new!(Degrees; degrees);
impl_into! {
    Degrees: 0 = value * R2D;
    Degrees: 0 * D2R;
    Degrees => Radians;
    Degrees => Revolutions: 0 = 0 / RV2D;
    Degrees => DegreesArcMinutes: Left = 0;
    Degrees => DegreesArcMinutesSeconds: Left = 0;
    Degrees => ArcMinutes: 0 = 0 * 60.0;
    Degrees => ArcMinutesSeconds: Left = 0 * 60.0;
    Degrees => ArcSeconds: 0 = 0 * 3600.0;
    Degrees => Hours: 0 = 0 / TA;
    Degrees => HoursMinutes: Left = 0 / TA;
    Degrees => HoursMinutesSeconds: Left = 0 / TA;
    Degrees => Minutes: 0 = 0 * TM;
    Degrees => MinutesSeconds: Left = 0 * TM;
    Degrees => Seconds: 0 = 0 * TS;
}

angle_serialize!(Degrees);
angle_deserialize!(Degrees);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct DegreesArcMinutes;

angle_new!(DegreesArcMinutes; degrees, arc_minutes);
impl_into! {
    DegreesArcMinutes: Left = value * R2D;
    DegreesArcMinutes: Left * D2R;
    DegreesArcMinutes => Radians;
    DegreesArcMinutes => Revolutions: 0 = Left / RV2D;
    DegreesArcMinutes => Degrees: 0 = Left;
    DegreesArcMinutes => DegreesArcMinutesSeconds: Middle = Right;
    DegreesArcMinutes => ArcMinutes: 0 = Right;
    DegreesArcMinutes => ArcMinutesSeconds: Left = Right;
    DegreesArcMinutes => ArcSeconds: 0 = Right * 60.0;
    DegreesArcMinutes => Hours: 0 = Left / TA;
    DegreesArcMinutes => HoursMinutes: Right = Right / TA;
    DegreesArcMinutes => HoursMinutesSeconds: Middle = Right / TA;
    DegreesArcMinutes => Minutes: 0 = Right / TA;
    DegreesArcMinutes => MinutesSeconds: Left = Right / TA;
    DegreesArcMinutes => Seconds: 0 = Right * TM;
}

angle_serialize!(DegreesArcMinutes);
angle_deserialize!(DegreesArcMinutes);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct DegreesArcMinutesSeconds;

angle_new!(DegreesArcMinutesSeconds; degrees, arc_minutes, arc_seconds);
impl_into! {
    DegreesArcMinutesSeconds: Left = value * R2D;
    DegreesArcMinutesSeconds: Left * D2R;
    DegreesArcMinutesSeconds => Radians;
    DegreesArcMinutesSeconds => Revolutions: 0 = Left / RV2D;
    DegreesArcMinutesSeconds => Degrees: 0 = Left;
    DegreesArcMinutesSeconds => DegreesArcMinutes: Right = Middle;
    DegreesArcMinutesSeconds => ArcMinutes: 0 = Middle;
    DegreesArcMinutesSeconds => ArcMinutesSeconds: Right = Right;
    DegreesArcMinutesSeconds => ArcSeconds: 0 = Right;
    DegreesArcMinutesSeconds => Hours: 0 = Left / TA;
    DegreesArcMinutesSeconds => HoursMinutes: Right = Middle / TA;
    DegreesArcMinutesSeconds => HoursMinutesSeconds: Right = Right / TA;
    DegreesArcMinutesSeconds => Minutes: 0 = Middle / TA;
    DegreesArcMinutesSeconds => MinutesSeconds: Right = Right / TA;
    DegreesArcMinutesSeconds => Seconds: 0 = Right / TA;
}

angle_serialize!(DegreesArcMinutesSeconds);
angle_deserialize!(DegreesArcMinutesSeconds);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcMinutes;

angle_new!(ArcMinutes; arc_minutes);
impl_into! {
    ArcMinutes: 0 = value * R2AM;
    ArcMinutes: 0 * AM2R;
    ArcMinutes => Radians;
    ArcMinutes => Revolutions: 0 = 0 / RV2AM;
    ArcMinutes => Degrees: 0 = 0 / 60.0;
    ArcMinutes => DegreesArcMinutes: Right = 0;
    ArcMinutes => DegreesArcMinutesSeconds: Middle = 0;
    ArcMinutes => ArcMinutesSeconds: Left = 0;
    ArcMinutes => ArcSeconds: 0 = 0 * 60.0;
    ArcMinutes => Hours: 0 = 0 / TMM;
    ArcMinutes => HoursMinutes: Right = 0 / TA;
    ArcMinutes => HoursMinutesSeconds: Middle = 0 / TA;
    ArcMinutes => Minutes: 0 = 0 / TA;
    ArcMinutes => MinutesSeconds: Left = 0 / TA;
    ArcMinutes => Seconds: 0 = 0 * TM;
}

angle_serialize!(ArcMinutes);
angle_deserialize!(ArcMinutes);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcMinutesSeconds;

angle_new!(ArcMinutesSeconds; arc_minutes, arc_seconds);
impl_into! {
    ArcMinutesSeconds: Left = value * R2AM;
    ArcMinutesSeconds: Left * AM2R;
    ArcMinutesSeconds => Radians;
    ArcMinutesSeconds => Revolutions: 0 = Left / RV2AM;
    ArcMinutesSeconds => Degrees: 0 = Left / 60.0;
    ArcMinutesSeconds => DegreesArcMinutes: Right = Left;
    ArcMinutesSeconds => DegreesArcMinutesSeconds: Right = Right;
    ArcMinutesSeconds => ArcMinutes: 0 = Left;
    ArcMinutesSeconds => ArcSeconds: 0 = Right;
    ArcMinutesSeconds => Hours: 0 = Left / TMM;
    ArcMinutesSeconds => HoursMinutes: Right = Left / TA;
    ArcMinutesSeconds => HoursMinutesSeconds: Right = Right / TA;
    ArcMinutesSeconds => Minutes: 0 = Left / TA;
    ArcMinutesSeconds => MinutesSeconds: Right = Right / TA;
    ArcMinutesSeconds => Seconds: 0 = Right / TA;
}

angle_serialize!(ArcMinutesSeconds);
angle_deserialize!(ArcMinutesSeconds);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcSeconds;

angle_new!(ArcSeconds; arc_seconds);
impl_into! {
    ArcSeconds: 0 = value * R2AS;
    ArcSeconds: 0 * AS2R;
    ArcSeconds => Radians;
    ArcSeconds => Revolutions: 0 = 0 / RV2AS;
    ArcSeconds => Degrees: 0 = 0 / 3600.0;
    ArcSeconds => DegreesArcMinutes: Right = 0 / 60.0;
    ArcSeconds => DegreesArcMinutesSeconds: Right = 0;
    ArcSeconds => ArcMinutes: 0 = 0 / 60.0;
    ArcSeconds => ArcMinutesSeconds: Left = 0 / 60.0;
    ArcSeconds => Hours: 0 = 0 / TMS;
    ArcSeconds => HoursMinutes: Right = 0 / TMM;
    ArcSeconds => HoursMinutesSeconds: Right = 0 / TA;
    ArcSeconds => Minutes: 0 = 0 / TMM;
    ArcSeconds => MinutesSeconds: Right = 0 / TA;
    ArcSeconds => Seconds: 0 = 0 / TA;
}

angle_serialize!(ArcSeconds);
angle_deserialize!(ArcSeconds);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Hours;

angle_new!(Hours; hours);
impl_into! {
    Hours: 0 = value * R2H;
    Hours: 0 * H2R;
    Hours => Radians;
    Hours => Revolutions: 0 = 0 / RV2H;
    Hours => Degrees: 0 = 0 * TA;
    Hours => DegreesArcMinutes: Left = 0 * TA;
    Hours => DegreesArcMinutesSeconds: Left = 0 * TA;
    Hours => ArcMinutes: 0 = 0 * TMM;
    Hours => ArcMinutesSeconds: Left = 0 * TMM;
    Hours => ArcSeconds: 0 = 0 * TMS;
    Hours => HoursMinutes: Left = 0;
    Hours => HoursMinutesSeconds: Left = 0;
    Hours => Minutes: 0 = 0 * 60.0;
    Hours => MinutesSeconds: Left = 0 * 60.0;
    Hours => Seconds: 0 = 0 * 3600.0;
}

angle_serialize!(Hours);
angle_deserialize!(Hours);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct HoursMinutes;

angle_new!(HoursMinutes; hours, minutes);
impl_into! {
    HoursMinutes: Left = value * R2H;
    HoursMinutes: Left * H2R;
    HoursMinutes => Radians;
    HoursMinutes => Revolutions: 0 = Left / RV2H;
    HoursMinutes => Degrees: 0 = Left * TA;
    HoursMinutes => DegreesArcMinutes: Right = Right * TA;
    HoursMinutes => DegreesArcMinutesSeconds: Middle = Right * TA;
    HoursMinutes => ArcMinutes: 0 = Right * TA;
    HoursMinutes => ArcMinutesSeconds: Left = Right * TA;
    HoursMinutes => ArcSeconds: 0 = Right * TMM;
    HoursMinutes => Hours: 0 = Left;
    HoursMinutes => HoursMinutesSeconds: Middle = Right;
    HoursMinutes => Minutes: 0 = Right;
    HoursMinutes => MinutesSeconds: Left = Right;
    HoursMinutes => Seconds: 0 = Right * 60.0;
}

angle_serialize!(HoursMinutes);
angle_deserialize!(HoursMinutes);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct HoursMinutesSeconds;

angle_new!(HoursMinutesSeconds; hours, minutes, seconds);
impl_into! {
    HoursMinutesSeconds: Left = value * R2H;
    HoursMinutesSeconds: Left * H2R;
    HoursMinutesSeconds => Radians;
    HoursMinutesSeconds => Revolutions: 0 = Left / RV2H;
    HoursMinutesSeconds => Degrees: 0 = Left * TA;
    HoursMinutesSeconds => DegreesArcMinutes: Right = Middle * TA;
    HoursMinutesSeconds => DegreesArcMinutesSeconds: Right = Right * TA;
    HoursMinutesSeconds => ArcMinutes: 0 = Middle * TA;
    HoursMinutesSeconds => ArcMinutesSeconds: Right = Right * TA;
    HoursMinutesSeconds => ArcSeconds: 0 = Right * TA;
    HoursMinutesSeconds => Hours: 0 = Left;
    HoursMinutesSeconds => HoursMinutes: Right = Middle;
    HoursMinutesSeconds => Minutes: 0 = Middle;
    HoursMinutesSeconds => MinutesSeconds: Right = Right;
    HoursMinutesSeconds => Seconds: 0 = Right;
}

angle_serialize!(HoursMinutesSeconds);
angle_deserialize!(HoursMinutesSeconds);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Minutes;

angle_new!(Minutes; minutes);
impl_into! {
    Minutes: 0 = value * R2M;
    Minutes: 0 * M2R;
    Minutes => Radians;
    Minutes => Revolutions: 0 = 0 / RV2M;
    Minutes => Degrees: 0 = 0 / TM;
    Minutes => DegreesArcMinutes: Right = 0 * TA;
    Minutes => DegreesArcMinutesSeconds: Middle = 0 * TA;
    Minutes => ArcMinutes: 0 = 0 * TA;
    Minutes => ArcMinutesSeconds: Left = 0 * TA;
    Minutes => ArcSeconds: 0 = 0 * TMM;
    Minutes => Hours: 0 = 0 / 60.0;
    Minutes => HoursMinutes: Right = 0;
    Minutes => HoursMinutesSeconds: Middle = 0;
    Minutes => MinutesSeconds: Left = 0;
    Minutes => Seconds: 0 = 0 * 60.0;
}

angle_serialize!(Minutes);
angle_deserialize!(Minutes);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct MinutesSeconds;

angle_new!(MinutesSeconds; minutes, seconds);
impl_into! {
    MinutesSeconds: Left = value * R2M;
    MinutesSeconds: Left * M2R;
    MinutesSeconds => Radians;
    MinutesSeconds => Revolutions: 0 = Left / RV2M;
    MinutesSeconds => Degrees: 0 = Left / TM;
    MinutesSeconds => DegreesArcMinutes: Right = Left * TA;
    MinutesSeconds => DegreesArcMinutesSeconds: Right = Right * TA;
    MinutesSeconds => ArcMinutes: 0 = Left * TA;
    MinutesSeconds => ArcMinutesSeconds: Right = Right * TA;
    MinutesSeconds => ArcSeconds: 0 = Right * TA;
    MinutesSeconds => Hours: 0 = Left / 60.0;
    MinutesSeconds => HoursMinutes: Right = Left;
    MinutesSeconds => HoursMinutesSeconds: Right = Right;
    MinutesSeconds => Minutes: 0 = Left;
    MinutesSeconds => Seconds: 0 = Right;
}

angle_serialize!(MinutesSeconds);
angle_deserialize!(MinutesSeconds);


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Seconds;

angle_new!(Seconds; seconds);
impl_into! {
    Seconds: 0 = value * R2S;
    Seconds: 0 * S2R;
    Seconds => Radians;
    Seconds => Revolutions: 0 = 0 / RV2S;
    Seconds => Degrees: 0 = 0 / TS;
    Seconds => DegreesArcMinutes: Right = 0 / TM;
    Seconds => DegreesArcMinutesSeconds: Right = 0 * TA;
    Seconds => ArcMinutes: 0 = 0 / TM;
    Seconds => ArcMinutesSeconds: Right = 0 * TA;
    Seconds => ArcSeconds: 0 = 0 * TA;
    Seconds => Hours: 0 = 0 / 3600.0;
    Seconds => HoursMinutes: Right = 0 / 60.0;
    Seconds => HoursMinutesSeconds: Right = 0;
    Seconds => Minutes: 0 = 0 / 60.0;
    Seconds => MinutesSeconds: Right = 0;
}

angle_serialize!(Seconds);
angle_deserialize!(Seconds);


#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform;

    use super::*;
    use crate::tests::{EPS, ITERATIONS};


    #[test]
    fn short_angle_test() {
        let short = ShortAngle(30, 30.0);
        let Left(t) = short.into();
        assert_relative_eq!(t, 30.5);
        let Right(t) = short.into();
        assert_relative_eq!(t, 30.0 * 60.0 + 30.0);

        let short: ShortAngle = Left(30.5).into();
        assert_eq!(short.0, 30);
        assert_relative_eq!(short.1, 30.0);
        let short: ShortAngle = Right(30.0 * 60.0 + 30.0).into();
        assert_eq!(short.0, 30);
        assert_relative_eq!(short.1, 30.0);

        let short = ShortAngle(-30, 36.0);
        let Left(t) = short.into();
        assert_relative_eq!(t, -30.6);
        let Right(t) = short.into();
        assert_relative_eq!(t, -(30.0 * 60.0 + 36.0));

        let short: ShortAngle = Left(-30.6).into();
        assert_eq!(short.0, -30);
        assert_relative_eq!(short.1, 36.0, epsilon = EPS);
        let short: ShortAngle = Right(-(30.0 * 60.0 + 36.0)).into();
        assert_eq!(short.0, -30);
        assert_relative_eq!(short.1, 36.0, epsilon = EPS);

        let short = ShortAngle(0, 36.0);
        let Left(t) = short.into();
        assert_relative_eq!(t, 0.6);
        let Right(t) = short.into();
        assert_relative_eq!(t, 36.0);

        let short: ShortAngle = Left(0.6).into();
        assert_eq!(short.0, 0);
        assert_relative_eq!(short.1, 36.0, epsilon = EPS);
        let short: ShortAngle = Right(36.0).into();
        assert_eq!(short.0, 0);
        assert_relative_eq!(short.1, 36.0, epsilon = EPS);

        let short = ShortAngle(0, -36.0);
        let Left(t) = short.into();
        assert_relative_eq!(t, -0.6);
        let Right(t) = short.into();
        assert_relative_eq!(t, -36.0);

        let short: ShortAngle = Left(-0.6).into();
        assert_eq!(short.0, 0);
        assert_relative_eq!(short.1, -36.0, epsilon = EPS);
        let short: ShortAngle = Right(-36.0).into();
        assert_eq!(short.0, 0);
        assert_relative_eq!(short.1, -36.0, epsilon = EPS);

        let mut rng = thread_rng();
        let side = Uniform::new(-360.0_f64, 360.0_f64);

        let left = Uniform::new(-360i32, 360i32);
        let right = Uniform::new(0.0_f64, 60.0_f64);

        for _ in 0..ITERATIONS {
            let left_value = rng.sample(side);

            let short: ShortAngle = Left(left_value).into();
            let Left(t) = short.into();
            assert_relative_eq!(t, left_value);

            let value1 = rng.sample(left);
            let value2 = rng.sample(right);

            let short = ShortAngle(value1, value2);
            let value: Left = short.into();
            let ShortAngle(t1, t2) = value.into();
            assert_eq!(t1, value1);
            assert_relative_eq!(t2, value2, epsilon = EPS);

            let right_value = 60.0 * left_value;

            let short: ShortAngle = Right(right_value).into();
            let Right(t) = short.into();
            assert_relative_eq!(t, right_value);
            let Left(t) = short.into();
            assert_relative_eq!(t, left_value);

            let short = ShortAngle(value1, value2);
            let value: Right = short.into();
            let ShortAngle(t1, t2) = value.into();
            assert_eq!(t1, value1);
            assert_relative_eq!(t2, value2, epsilon = EPS);
        }
    }

    #[test]
    fn long_angle_test() {
        let long = LongAngle(16, 14, 4.2);
        let Left(t) = long.into();
        assert_relative_eq!(t, 16.2345);
        let Middle(t) = long.into();
        assert_relative_eq!(t, 60.0 * 16.0 + 14.07);
        let Right(t) = long.into();
        assert_relative_eq!(t, 4.2 + 60.0 * (14.0 + 60.0 * 16.0));

        let long: LongAngle = Left(16.2345).into();
        assert_eq!(long.0, 16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Middle(60.0 * 16.0 + 14.07).into();
        assert_eq!(long.0, 16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Right(4.2 + 60.0 * (14.0 + 60.0 * 16.0)).into();
        assert_eq!(long.0, 16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long = LongAngle(-16, 14, 4.2);
        let Left(t) = long.into();
        assert_relative_eq!(t, -16.2345);
        let Middle(t) = long.into();
        assert_relative_eq!(t, -(60.0 * 16.0 + 14.07));
        let Right(t) = long.into();
        assert_relative_eq!(t, -(4.2 + 60.0 * (14.0 + 60.0 * 16.0)));

        let long: LongAngle = Left(-16.2345).into();
        assert_eq!(long.0, -16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Middle(-(60.0 * 16.0 + 14.07)).into();
        assert_eq!(long.0, -16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Right(-(4.2 + 60.0 * (14.0 + 60.0 * 16.0))).into();
        assert_eq!(long.0, -16);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long = LongAngle(0, 14, 4.2);
        let Left(t) = long.into();
        assert_relative_eq!(t, 0.2345);
        let Middle(t) = long.into();
        assert_relative_eq!(t, 14.07);
        let Right(t) = long.into();
        assert_relative_eq!(t, 4.2 + 60.0 * 14.0);

        let long: LongAngle = Left(0.2345).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Middle(14.07).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Right(4.2 + 60.0 * 14.0).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long = LongAngle(0, -14, 4.2);
        let Left(t) = long.into();
        assert_relative_eq!(t, -0.2345);
        let Middle(t) = long.into();
        assert_relative_eq!(t, -14.07);
        let Right(t) = long.into();
        assert_relative_eq!(t, -(4.2 + 60.0 * 14.0));

        let long: LongAngle = Left(-0.2345).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, -14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Middle(-14.07).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, -14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long: LongAngle = Right(-(4.2 + 60.0 * 14.0)).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, -14);
        assert_relative_eq!(long.2, 4.2, epsilon = EPS);

        let long = LongAngle(0, 0, 36.0);
        let Left(t) = long.into();
        assert_relative_eq!(t, 0.01);
        let Middle(t) = long.into();
        assert_relative_eq!(t, 0.6);
        let Right(t) = long.into();
        assert_relative_eq!(t, 36.0);

        let long: LongAngle = Left(0.01).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, 36.0, epsilon = EPS);

        let long: LongAngle = Middle(0.6).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, 36.0, epsilon = EPS);

        let long: LongAngle = Right(36.0).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, 36.0, epsilon = EPS);

        let long = LongAngle(0, 0, -36.0);
        let Left(t) = long.into();
        assert_relative_eq!(t, -0.01);
        let Middle(t) = long.into();
        assert_relative_eq!(t, -0.6);
        let Right(t) = long.into();
        assert_relative_eq!(t, -36.0);

        let long: LongAngle = Left(-0.01).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, -36.0, epsilon = EPS);

        let long: LongAngle = Middle(-0.6).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, -36.0, epsilon = EPS);

        let long: LongAngle = Right(-36.0).into();
        assert_eq!(long.0, 0);
        assert_eq!(long.1, 0);
        assert_relative_eq!(long.2, -36.0, epsilon = EPS);

        let mut rng = thread_rng();
        let side = Uniform::new(-360.0_f64, 360.0_f64);

        let left = Uniform::new(-360i32, 360i32);
        let middle = Uniform::new(0i8, 60i8);
        let right = Uniform::new(0.0_f64, 60.0_f64);

        for _ in 0..ITERATIONS {
            let left_value = rng.sample(side);

            let long: LongAngle = Left(left_value).into();
            let Left(t) = long.into();
            assert_relative_eq!(t, left_value);

            let value1 = rng.sample(left);
            let value2 = rng.sample(middle);
            let value3 = rng.sample(right);

            let long = LongAngle(value1, value2, value3);
            let value: Left = long.into();
            let LongAngle(t1, t2, t3) = value.into();
            assert_eq!(t1, value1);
            assert_eq!(t2, value2);
            assert_relative_eq!(t3, value3, epsilon = EPS);

            let middle_value = 60.0 * left_value;

            let long: LongAngle = Middle(middle_value).into();
            let Middle(t) = long.into();
            assert_relative_eq!(t, middle_value);
            let Left(t) = long.into();
            assert_relative_eq!(t, left_value);

            let long = LongAngle(value1, value2, value3);
            let value: Middle = long.into();
            let LongAngle(t1, t2, t3) = value.into();
            assert_eq!(t1, value1);
            assert_eq!(t2, value2);
            assert_relative_eq!(t3, value3, epsilon = EPS);

            let right_value = 60.0 * middle_value;

            let long: LongAngle = Right(right_value).into();
            let Right(t) = long.into();
            assert_relative_eq!(t, right_value);
            let Middle(t) = long.into();
            assert_relative_eq!(t, middle_value);
            let Left(t) = long.into();
            assert_relative_eq!(t, left_value);

            let long = LongAngle(value1, value2, value3);
            let value: Right = long.into();
            let LongAngle(t1, t2, t3) = value.into();
            assert_eq!(t1, value1);
            assert_eq!(t2, value2);
            assert_relative_eq!(t3, value3, epsilon = EPS);
        }
    }
}