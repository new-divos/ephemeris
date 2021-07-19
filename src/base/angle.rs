use std::cmp::Ordering;
use std::convert;
use std::fmt;
use std::marker::PhantomData;

use crate::base::Real;
use crate::base::consts::{PI2, D2R, R2D, R2AM, AM2R, R2AS, AS2R,
                          H2R, R2H, M2R, R2M, S2R, R2S};


const RV2D: f64 = 360.0;
const RV2AM: f64 = RV2D * 60.0;
const RV2AS: f64 = RV2AM * 60.0;

const RV2H: f64 = 24.0;
const RV2M: f64 = RV2H * 60.0;
const RV2S: f64 = RV2M * 60.0;

const TA: f64 = 15.0;
const TM: f64 = 4.0;     // TM = 60.0 / TA;
const TS: f64 = 240.0;   // TS = 3600.0 / TA;

const TMM: f64 = TA * 60.0;
const TMS: f64 = TMM * 60.0;


// ########################################################
// # Type Left
// ########################################################

/// The left value of a composed angle value
#[derive(Copy, Clone, Debug, PartialEq)]
struct Left(f64);


// ########################################################
// # Type Middle
// ########################################################

/// The middle value of a composed angle value
#[derive(Copy, Clone, Debug, PartialEq)]
struct Middle(f64);


// ########################################################
// # Type Right
// ########################################################

/// The right value of a composed angle value
#[derive(Copy, Clone, Debug, PartialEq)]
struct Right(f64);


// ########################################################
// # Type Sign
// ########################################################

/// The sign of the angle
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}

fn sign(value: f64) -> Sign {
    if value > 0.0 {
        Sign::Positive
    } else if value < 0.0 {
        Sign::Negative
    } else {
        Sign::Zero
    }
}


// ########################################################
// # Type ShortAngle
// ########################################################

/// The two components angle value
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

// ########################################################
// # Type LongAngle
// ########################################################

/// The three components angle value
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

// ########################################################
// # Trait AngleWrapper
// ########################################################

pub trait AngleMeta {
    type Item: Copy;

    const ROTATION: f64;
}


// ########################################################
// # Trait AngleWrapper
// ########################################################

pub trait AngleNormalizer {
    fn normalize(&self) -> Self;
    fn translate(&self, n: i32) -> Self;
}

// ########################################################
// # Type Angle
// ########################################################

#[derive(Clone, Copy)]
pub struct Angle<T: AngleMeta + Copy>
(
    <T as AngleMeta>::Item,
    PhantomData<T>
);


macro_rules! angle_new {
    ($t:ty; $v:ident) => {
        impl fmt::Debug for Angle<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(concat!("Angle<", stringify!($t), ">"))
                    .field(stringify!($v), &self.0)
                    .finish()
            }
        }

        impl Angle<$t> {
            #[inline]
            pub fn $v(&self) -> f64 {
                self.0
            }

            #[inline]
            pub fn sign(&self) -> Sign {
                sign(self.0)
            }

            #[inline]
            pub fn new($v: f64) -> Self {
                Self($v, PhantomData::<$t>)
            }
        }
    };
    ($t:ty; $v1:ident, $v2:ident) => {
        impl fmt::Debug for Angle<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(concat!("Angle<", stringify!($t), ">"))
                    .field(stringify!($v1), &self.0.0)
                    .field(stringify!($v2), &self.0.1)
                    .finish()
            }
        }

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
        impl fmt::Debug for Angle<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(concat!("Angle<", stringify!($t), ">"))
                    .field(stringify!($v1), &self.0.0)
                    .field(stringify!($v2), &self.0.1)
                    .field(stringify!($v3), &self.0.2)
                    .finish()
            }
        }

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


macro_rules! angle_from {
    (@impl_from $td:ty: $ts:ty => (0 = 0 * $c:expr)) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                Self(angle.0 * $c, PhantomData::<$td>)
            }
        }
    };

    (@impl_from $td:ty: $ts:ty => (0 = 0 / $c:expr)) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                Self(angle.0 / $c, PhantomData::<$td>)
            }
        }
    };

    (@impl_from $td:ty: $ts:ty => ($m:ident = 0 * $c:expr)) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                Self($m(angle.0 * $c).into(), PhantomData::<$td>)
            }
        }
    };

    ($td:ty:) => {};

    ($td:ty: Radians => (0 = 0 * $c:expr), $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(radians: f64) -> Self {
                Self(radians * $c, PhantomData::<$td>)
            }
        }

        angle_from!(
            @impl_from
            $td: Radians => (0 = 0 * $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => (0 = 0 * $c:expr), $($tail:tt)*) => {
        angle_from!(
            @impl_from
            $td: $ts => (0 = 0 * $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: Radians => (0 = 0 / $c:expr), $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(radians: f64) -> Self {
                Self(radians / $c, PhantomData::<$td>)
            }
        }

        angle_from!(
            @impl_from
            $td: Radians => (0 = 0 / $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => (0 = 0 / $c:expr), $($tail:tt)*) => {
        angle_from!(
            @impl_from
            $td: $ts => (0 = 0 / $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($m:ident = 0), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                Self($m(angle.0).into(), PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: Radians => ($m:ident = 0 * $c:expr), $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(radians: f64) -> Self {
                Self($m(radians * $c).into(), PhantomData::<$td>)
            }
        }

        angle_from!(
            @impl_from
            $td: Radians => ($m = 0 * $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($m:ident = 0 * $c:expr), $($tail:tt)*) => {
        angle_from!(
            @impl_from
            $td: $ts => ($m = 0 * $c)
        );

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($m:ident = 0 / $c:expr), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                Self($m(angle.0 / $c).into(), PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => (0 = $m:ident), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $m(value) = angle.0.into();
                Self(value, PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => (0 = $m:ident * $c:expr), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $m(value) = angle.0.into();
                Self(value * $c, PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => (0 = $m:ident / $c:expr), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $m(value) = angle.0.into();
                Self(value / $c, PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($md:ident = $ms:ident), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $ms(value) = angle.0.into();
                Self($md(value).into(), PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($md:ident = $ms:ident * $c:expr), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $ms(value) = angle.0.into();
                Self($md(value * $c).into(), PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };

    ($td:ty: $ts:ty => ($md:ident = $ms:ident / $c:expr), $($tail:tt)*) => {
        impl convert::From<Angle<$ts>> for Angle<$td> {
            #[inline]
            fn from(angle: Angle<$ts>) -> Self {
                let $ms(value) = angle.0.into();
                Self($md(value / $c).into(), PhantomData::<$td>)
            }
        }

        angle_from!($td: $($tail)*);
    };
}


macro_rules! angle_raw {
    ($ts:ty: 0 * $c:expr) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                self.0 * $c
            }
        }
    };

    ($ts:ty: $m:ident * $c:expr) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                let $m(value) = self.0.into();
                value * $c
            }
        }
    }
}


impl<T> convert::Into<(Sign, f64)> for Angle<T>
    where
        T: AngleMeta<Item=f64> + Copy,
{
    #[inline]
    fn into(self) -> (Sign, f64) {
        (sign(self.0), self.0.abs())
    }
}

impl<T> convert::Into<(Sign, i32, f64)> for Angle<T>
    where
        T: AngleMeta<Item=ShortAngle> + Copy,
{
    #[inline]
    fn into(self) -> (Sign, i32, f64) {
        (self.0.sign(), self.0.0.abs(), self.0.1.abs())
    }
}

impl<T> convert::Into<(Sign, i32, i32, f64)> for Angle<T>
    where
        T: AngleMeta<Item=LongAngle> + Copy,
{
    #[inline]
    fn into(self) -> (Sign, i32, i32, f64) {
        (self.0.sign(), self.0.0.abs(), (self.0.1 as i32).abs(), self.0.2.abs())
    }
}


// ########################################################
// # Type Radians
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = "PI2"]
pub struct Radians;

angle_new!(Radians; radians);
angle_serialize!(Radians);
angle_deserialize!(Radians);

angle_from! {
    Radians:
        Revolutions => (0 = 0 * PI2),
        Degrees => (0 = 0 * D2R),
        DegreesArcMinutes => (0 = Left * D2R),
        DegreesArcMinutesSeconds => (0 = Left * D2R),
        ArcMinutes => (0 = 0 * AM2R),
        ArcMinutesSeconds => (0 = Left * AM2R),
        ArcSeconds => (0 = 0 * AS2R),
        Hours => (0 = 0 * H2R),
        HoursMinutes => (0 = Left * H2R),
        HoursMinutesSeconds => (0 = Left * H2R),
        Minutes => (0 = 0 * M2R),
        MinutesSeconds => (0 = Left * M2R),
        Seconds => (0 = 0 * S2R),
}

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


// ########################################################
// # Type Revolutions
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 1.0]
pub struct Revolutions;

angle_new!(Revolutions; revolutions);
angle_serialize!(Revolutions);
angle_deserialize!(Revolutions);

angle_from! {
    Revolutions:
        Radians => (0 = 0 / PI2),
        Degrees => (0 = 0 / RV2D),
        DegreesArcMinutes => (0 = Left / RV2D),
        DegreesArcMinutesSeconds => (0 = Left / RV2D),
        ArcMinutes => (0 = 0 / RV2AM),
        ArcMinutesSeconds => (0 = Left / RV2AM),
        ArcSeconds => (0 = 0 / RV2AS),
        Hours => (0 = 0 / RV2H),
        HoursMinutes => (0 = Left / RV2H),
        HoursMinutesSeconds => (0 = Left / RV2H),
        Minutes => (0 = 0 / RV2M),
        MinutesSeconds => (0 = Left / RV2M),
        Seconds => (0 = 0 / RV2S),
}

angle_raw!(Revolutions: 0 * PI2);


// +-------------------------------------------------------
// | Type Angle<Revolutions>
// +-------------------------------------------------------

impl AngleNormalizer for Angle<Revolutions> {
    #[inline]
    fn normalize(&self) -> Self {
        Self(self.0.frac(), PhantomData::<Revolutions>)
    }

    #[inline]
    fn translate(&self, n: i32) -> Self {
        Self(self.0 + (n as f64), PhantomData::<Revolutions>)
    }
}


// ########################################################
// # Type Degrees
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 360.0]
pub struct Degrees;

angle_new!(Degrees; degrees);
angle_serialize!(Degrees);
angle_deserialize!(Degrees);

angle_from! {
    Degrees:
        Radians => (0 = 0 * R2D),
        Revolutions => (0 = 0 * RV2D),
        DegreesArcMinutes => (0 = Left),
        DegreesArcMinutesSeconds => (0 = Left),
        ArcMinutes => (0 = 0 / 60.0),
        ArcMinutesSeconds => (0 = Left / 60.0),
        ArcSeconds => (0 = 0 / 3600.0),
        Hours => (0 = 0 * TA),
        HoursMinutes => (0 = Left * TA),
        HoursMinutesSeconds => (0 = Left * TA),
        Minutes => (0 = 0 / TM),
        MinutesSeconds => (0 = Left / TM),
        Seconds => (0 = 0 / TS),
}

angle_raw!(Degrees: 0 * D2R);


// ########################################################
// # Type DegreesArcMinutes
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 360.0]
pub struct DegreesArcMinutes;

angle_new!(DegreesArcMinutes; degrees, arc_minutes);
angle_serialize!(DegreesArcMinutes);
angle_deserialize!(DegreesArcMinutes);

angle_from! {
    DegreesArcMinutes:
        Radians => (Left = 0 * R2D),
        Revolutions => (Left = 0 * RV2D),
        Degrees => (Left = 0),
        DegreesArcMinutesSeconds => (Right = Middle),
        ArcMinutes => (Right = 0),
        ArcMinutesSeconds => (Right = Left),
        ArcSeconds => (Right = 0 / 60.0),
        Hours => (Left = 0 * TA),
        HoursMinutes => (Left = Left * TA),
        HoursMinutesSeconds => (Left = Left * TA),
        Minutes => (Right = 0 * TA),
        MinutesSeconds => (Right = Left * TA),
        Seconds => (Right = 0 / TM),
}

angle_raw!(DegreesArcMinutes: Left * D2R);


// ########################################################
// # Type DegreesArcMinutesSeconds
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 360.0]
pub struct DegreesArcMinutesSeconds;

angle_new!(DegreesArcMinutesSeconds; degrees, arc_minutes, arc_seconds);
angle_serialize!(DegreesArcMinutesSeconds);
angle_deserialize!(DegreesArcMinutesSeconds);

angle_from! {
    DegreesArcMinutesSeconds:
        Radians => (Left = 0 * R2D),
        Revolutions => (Left = 0 * RV2D),
        Degrees => (Left = 0),
        DegreesArcMinutes => (Middle = Right),
        ArcMinutes => (Middle = 0),
        ArcMinutesSeconds => (Right = Right),
        ArcSeconds => (Right = 0),
        Hours => (Left = 0 * TA),
        HoursMinutes => (Left = Left * TA),
        HoursMinutesSeconds => (Left = Left * TA),
        Minutes => (Middle = 0 * TA),
        MinutesSeconds => (Right = Right * TA),
        Seconds => (Right = 0 * TA),
}

angle_raw!(DegreesArcMinutesSeconds: Left * D2R);


// ########################################################
// # Type ArcMinutes
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 21600.0]
pub struct ArcMinutes;

angle_new!(ArcMinutes; arc_minutes);
angle_serialize!(ArcMinutes);
angle_deserialize!(ArcMinutes);

angle_from! {
    ArcMinutes:
        Radians => (0 = 0 * R2AM),
        Revolutions => (0 = 0 * RV2AM),
        Degrees => (0 = 0 * 60.0),
        DegreesArcMinutes => (0 = Right),
        DegreesArcMinutesSeconds => (0 = Middle),
        ArcMinutesSeconds => (0 = Left),
        ArcSeconds => (0 = 0 / 60.0),
        Hours => (0 = 0 * TMM),
        HoursMinutes => (0 = Right * TA),
        HoursMinutesSeconds => (0 = Middle * TA),
        Minutes => (0 = 0 * TA),
        MinutesSeconds => (0 = Left * TA),
        Seconds => (0 = 0 / TM),
}

angle_raw!(ArcMinutes: 0 * AM2R);


// ########################################################
// # Type ArcMinutesSeconds
// ########################################################


#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 21600.0]
pub struct ArcMinutesSeconds;

angle_new!(ArcMinutesSeconds; arc_minutes, arc_seconds);
angle_serialize!(ArcMinutesSeconds);
angle_deserialize!(ArcMinutesSeconds);

angle_from! {
    ArcMinutesSeconds:
        Radians => (Left = 0 * R2AM),
        Revolutions => (Left = 0 * RV2AM),
        Degrees => (Left = 0 * 60.0),
        DegreesArcMinutes => (Left = Right),
        DegreesArcMinutesSeconds => (Right = Right),
        ArcMinutes => (Left = 0),
        ArcSeconds => (Right = 0),
        Hours => (Left = 0 * TMM),
        HoursMinutes => (Left = Right * TA),
        HoursMinutesSeconds => (Right = Right * TA),
        Minutes => (Left = 0 * TA),
        MinutesSeconds => (Right = Right * TA),
        Seconds => (Right = 0 * TA),
}

angle_raw!(ArcMinutesSeconds: Left * AM2R);


// ########################################################
// # Type ArcSeconds
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 1296000.0]
pub struct ArcSeconds;

angle_new!(ArcSeconds; arc_seconds);
angle_serialize!(ArcSeconds);
angle_deserialize!(ArcSeconds);

angle_from! {
    ArcSeconds:
        Radians => (0 = 0 * R2AS),
        Revolutions => (0 = 0 * RV2AS),
        Degrees => (0 = 0 * 3600.0),
        DegreesArcMinutes => (0 = Right * 60.0),
        DegreesArcMinutesSeconds => (0 = Right),
        ArcMinutes => (0 = 0 * 60.0),
        ArcMinutesSeconds => (0 = Right),
        Hours => (0 = 0 * TMS),
        HoursMinutes => (0 = Right * TMM),
        HoursMinutesSeconds => (0 = Right * TA),
        Minutes => (0 = 0 * TMM),
        MinutesSeconds => (0 = Right * TA),
        Seconds => (0 = 0 * TA),
}

angle_raw!(ArcSeconds: 0 * AS2R);


// ########################################################
// # Type Hours
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 24.0]
pub struct Hours;

angle_new!(Hours; hours);
angle_serialize!(Hours);
angle_deserialize!(Hours);

angle_from! {
    Hours:
        Radians => (0 = 0 * R2H),
        Revolutions => (0 = 0 * RV2H),
        Degrees => (0 = 0 / TA),
        DegreesArcMinutes => (0 = Left / TA),
        DegreesArcMinutesSeconds => (0 = Left / TA),
        ArcMinutes => (0 = 0 / TMM),
        ArcMinutesSeconds => (0 = Left / TMM),
        ArcSeconds => (0 = 0 / TMS),
        HoursMinutes => (0 = Left),
        HoursMinutesSeconds => (0 = Left),
        Minutes => (0 = 0 / 60.0),
        MinutesSeconds => (0 = Left / 60.0),
        Seconds => (0 = 0 / 3600.0),
}

angle_raw!(Hours: 0 * H2R);


// ########################################################
// # Type HoursMinutes
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 24.0]
pub struct HoursMinutes;

angle_new!(HoursMinutes; hours, minutes);
angle_serialize!(HoursMinutes);
angle_deserialize!(HoursMinutes);

angle_from! {
    HoursMinutes:
        Radians => (Left = 0 * R2H),
        Revolutions => (Left = 0 * RV2H),
        Degrees => (Left = 0 / TA),
        DegreesArcMinutes => (Right = Right / TA),
        DegreesArcMinutesSeconds => (Right = Middle / TA),
        ArcMinutes => (Right = 0 / TA),
        ArcMinutesSeconds => (Right = Left / TA),
        ArcSeconds => (Right = 0 / TMM),
        Hours => (Left = 0),
        HoursMinutesSeconds => (Right = Middle),
        Minutes => (Right = 0),
        MinutesSeconds => (Right = Left),
        Seconds => (Right = 0 / 60.0),
}

angle_raw!(HoursMinutes: Left * H2R);


// ########################################################
// # Type HoursMinutesSeconds
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 24.0]
pub struct HoursMinutesSeconds;

angle_new!(HoursMinutesSeconds; hours, minutes, seconds);
angle_serialize!(HoursMinutesSeconds);
angle_deserialize!(HoursMinutesSeconds);

angle_from! {
    HoursMinutesSeconds:
        Radians => (Left = 0 * R2H),
        Revolutions => (Left = 0 * RV2H),
        Degrees => (Left = 0 / TA),
        DegreesArcMinutes => (Middle = Right / TA),
        DegreesArcMinutesSeconds => (Right = Right / TA),
        ArcMinutes => (Middle = 0 / TA),
        ArcMinutesSeconds => (Right = Right / TA),
        ArcSeconds => (Right = 0 / TA),
        Hours => (Left = 0),
        HoursMinutes => (Middle = Right),
        Minutes => (Middle = 0),
        MinutesSeconds => (Right = Right),
        Seconds => (Right = 0),
}

angle_raw!(HoursMinutesSeconds: Left * H2R);


// ########################################################
// # Type Minutes
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 1440.0]
pub struct Minutes;

angle_new!(Minutes; minutes);
angle_serialize!(Minutes);
angle_deserialize!(Minutes);

angle_from! {
    Minutes:
        Radians => (0 = 0 * R2M),
        Revolutions => (0 = 0 * RV2M),
        Degrees => (0 = 0 * TM),
        DegreesArcMinutes => (0 = Right / TA),
        DegreesArcMinutesSeconds => (0 = Middle / TA),
        ArcMinutes => (0 = 0 / TA),
        ArcMinutesSeconds => (0 = Left / TA),
        ArcSeconds => (0 = 0 / TMM),
        Hours => (0 = 0 * 60.0),
        HoursMinutes => (0 = Right),
        HoursMinutesSeconds => (0 = Middle),
        MinutesSeconds => (0 = Left),
        Seconds => (0 = 0 / 60.0),
}

angle_raw!(Minutes: 0 * M2R);


// ########################################################
// # Type MinutesSeconds
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 1440.0]
pub struct MinutesSeconds;

angle_new!(MinutesSeconds; minutes, seconds);
angle_serialize!(MinutesSeconds);
angle_deserialize!(MinutesSeconds);

angle_from! {
    MinutesSeconds:
        Radians => (Left = 0 * R2M),
        Revolutions => (Left = 0 * RV2M),
        Degrees => (Left = 0 * TM),
        DegreesArcMinutes => (Left = Right / TA),
        DegreesArcMinutesSeconds => (Right = Right / TA),
        ArcMinutes => (Left = 0 / TA),
        ArcMinutesSeconds => (Right = Right / TA),
        ArcSeconds => (Right = 0 / TA),
        Hours => (Left = 0 * 60.0),
        HoursMinutes => (Left = Right),
        HoursMinutesSeconds => (Right = Right),
        Minutes => (Left = 0),
        Seconds => (Right = 0),
}

angle_raw!(MinutesSeconds: Left * M2R);


// ########################################################
// # Type Seconds
// ########################################################

#[derive(AngleMeta, Clone, Copy, Debug)]
#[rotation = 86400.0]
pub struct Seconds;

angle_new!(Seconds; seconds);
angle_serialize!(Seconds);
angle_deserialize!(Seconds);

angle_from! {
    Seconds:
        Radians => (0 = 0 * R2S),
        Revolutions => (0 = 0 * RV2S),
        Degrees => (0 = 0 * TS),
        DegreesArcMinutes => (0 = Right * TM),
        DegreesArcMinutesSeconds => (0 = Right / TA),
        ArcMinutes => (0 = 0 * TM),
        ArcMinutesSeconds => (0 = Right / TA),
        ArcSeconds => (0 = 0 / TA),
        Hours => (0 = 0 * 3600.0),
        HoursMinutes => (0 = Right * 60.0),
        HoursMinutesSeconds => (0 = Right),
        Minutes => (0 = 0 * 60.0),
        MinutesSeconds => (0 = Right),
}

angle_raw!(Seconds: 0 * S2R);


// ########################################################
// # Tests
// ########################################################

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform;

    use super::*;
    use crate::tests::{EPS, ITERATIONS};

    #[test]
    fn rotation_test() {
        assert_eq!(Radians::ROTATION, PI2);
        assert_eq!(Revolutions::ROTATION, 1.0);
        assert_eq!(Degrees::ROTATION, RV2D);
        assert_eq!(DegreesArcMinutes::ROTATION, RV2D);
        assert_eq!(DegreesArcMinutesSeconds::ROTATION, RV2D);
        assert_eq!(ArcMinutes::ROTATION, RV2AM);
        assert_eq!(ArcMinutesSeconds::ROTATION, RV2AM);
        assert_eq!(ArcSeconds::ROTATION, RV2AS);
        assert_eq!(Hours::ROTATION, RV2H);
        assert_eq!(HoursMinutes::ROTATION, RV2H);
        assert_eq!(HoursMinutesSeconds::ROTATION, RV2H);
        assert_eq!(Minutes::ROTATION, RV2M);
        assert_eq!(MinutesSeconds::ROTATION, RV2M);
        assert_eq!(Seconds::ROTATION, RV2S);
    }


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