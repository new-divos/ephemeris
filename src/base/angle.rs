use std::cmp::Ordering;
use std::convert;
use std::marker::PhantomData;

use crate::base::consts::{PI2, D2R, R2D, R2AM, AM2R, R2AS, AS2R,
                          H2R, R2H, M2R, R2M, S2R, R2S};


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

pub trait AngleSign {
    fn sign(&self) -> Sign;
}


#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SimpleAngle(f64);

impl convert::From<f64> for SimpleAngle {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl convert::Into<f64> for SimpleAngle {
    #[inline]
    fn into(self) -> f64 {
        self.0
    }
}

impl convert::Into<(Sign, f64)> for SimpleAngle {
    #[inline]
    fn into(self) -> (Sign, f64) {
        (self.sign(), self.0.abs())
    }
}

impl AngleSign for SimpleAngle {
    fn sign(&self) -> Sign {
        if self.0 == 0.0 {
            Sign::Zero
        } else if self.0 < 0.0 {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }
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

impl AngleSign for ShortAngle {
    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0.0 {
            Sign::Zero
        } else if Self::is_negative(self.0, self.1) {
            Sign::Negative
        } else {
            Sign::Positive
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

impl AngleSign for LongAngle {
    fn sign(&self) -> Sign {
        if self.0 == 0 && self.1 == 0 && self.2 == 0.0 {
            Sign::Zero
        } else if Self::is_negative(self.0, self.1 as i32, self.2) {
            Sign::Negative
        } else {
            Sign::Positive
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


impl<T> convert::Into<(Sign, f64)> for Angle<T>
    where T: AngleMapper<Item=SimpleAngle> + Copy
{
    #[inline]
    fn into(self) -> (Sign, f64) {
        self.0.into()
    }
}

impl<T> convert::Into<(i32, f64)> for Angle<T>
    where T: AngleMapper<Item=ShortAngle> + Copy
{
    #[inline]
    fn into(self) -> (i32, f64) {
        self.0.into()
    }
}

impl<T> convert::Into<(Sign, i32, f64)> for Angle<T>
    where T: AngleMapper<Item=ShortAngle> + Copy
{
    #[inline]
    fn into(self) -> (Sign, i32, f64) {
        self.0.into()
    }
}

impl<T> convert::Into<(i32, i32, f64)> for Angle<T>
    where T: AngleMapper<Item=LongAngle> + Copy
{
    #[inline]
    fn into(self) -> (i32, i32, f64) {
        self.0.into()
    }
}

impl<T> convert::Into<(Sign, i32, i32, f64)> for Angle<T>
    where T: AngleMapper<Item=LongAngle> + Copy
{
    #[inline]
    fn into(self) -> (Sign, i32, i32, f64) {
        self.0.into()
    }
}


impl<T> Angle<T>
    where T: AngleMapper + Copy,
          <T as AngleMapper>::Item: AngleSign
{
    #[inline]
    pub fn sign(&self) -> Sign {
        self.0.sign()
    }
}


macro_rules! impl_new {
    ($t:ty; $v:ident) => {
        impl Angle<$t> {
            #[inline]
            pub fn $v(&self) -> f64 {
                self.0.0
            }

            #[inline]
            pub fn new($v: f64) -> Self {
                Self(SimpleAngle::from($v), PhantomData::<$t>{})
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
            pub fn new($v1: i32, $v2: f64) -> Self {
                Self(ShortAngle::new($v1, $v2), PhantomData::<$t>{})
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
            pub fn new($v1: i32, $v2: i32, $v3: f64) -> Self {
                Self(LongAngle::new($v1, $v2, $v3), PhantomData::<$t>{})
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
                Self(
                    SimpleAngle::from(value * $c),
                    PhantomData::<$td>{}
                )
            }
        }

        impl_into!($($tail)*);
    };
    ($td:ty: 0 = value / $c:expr; $($tail:tt)*) => {
        impl convert::From<f64> for Angle<$td> {
            #[inline]
            fn from(value: f64) -> Self {
                Self(
                    SimpleAngle::from(value / $c),
                    PhantomData::<$td>{}
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
                    $m(value * $c).into(),
                    PhantomData::<$td>{}
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
                    PhantomData::<$td>{}
                )
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty: 0 * $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                self.0.0 * $c
            }
        }

        impl_into!($($tail)*);
    };
    ($ts:ty: 0 / $c:expr; $($tail:tt)*) => {
        impl convert::Into<f64> for Angle<$ts> {
            #[inline]
            fn into(self) -> f64 {
                self.0.0 / $c
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
                Angle::<Radians>(self.into(), PhantomData::<Radians>{})
            }
        }

        impl_into!($($tail)*);
    };
}


#[derive(Clone, Copy, Debug)]
pub struct Radians;

impl AngleMapper for Radians {
    type Item = f64;
}

impl convert::From<f64> for Angle<Radians> {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value, PhantomData::<Radians>{})
    }
}

impl convert::Into<f64> for Angle<Radians> {
    #[inline]
    fn into(self) -> f64 {
        self.0
    }
}

impl AngleSign for Angle<Radians> {
    #[inline]
    fn sign(&self) -> Sign {
        SimpleAngle(self.0).sign()
    }
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Revolutions;

impl_new!(Revolutions; revolutions);
impl_into! {
    Revolutions: 0 = value * PI2;
    Revolutions: 0 / PI2;
    Revolutions => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Degrees;

impl_new!(Degrees; degrees);
impl_into! {
    Degrees: 0 = value * R2D;
    Degrees: 0 * D2R;
    Degrees => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct DegreesArcMinutes;

impl_new!(DegreesArcMinutes; degrees, arc_minutes);
impl_into! {
    DegreesArcMinutes: Left = value * R2D;
    DegreesArcMinutes: Left * D2R;
    DegreesArcMinutes => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct DegreesArcMinutesSeconds;

impl_new!(DegreesArcMinutesSeconds; degrees, arc_minutes, arc_seconds);
impl_into! {
    DegreesArcMinutesSeconds: Left = value * R2D;
    DegreesArcMinutesSeconds: Left * D2R;
    DegreesArcMinutesSeconds => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcMinutes;

impl_new!(ArcMinutes; arc_minutes);
impl_into! {
    ArcMinutes: 0 = value * R2AM;
    ArcMinutes: 0 * AM2R;
    ArcMinutes => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcMinutesSeconds;

impl_new!(ArcMinutesSeconds; arc_minutes, arc_seconds);
impl_into! {
    ArcMinutesSeconds: Left = value * R2AM;
    ArcMinutesSeconds: Left * AM2R;
    ArcMinutesSeconds => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct ArcSeconds;

impl_new!(ArcSeconds; arc_seconds);
impl_into! {
    ArcSeconds: 0 = value * R2AS;
    ArcSeconds: 0 * AS2R;
    ArcSeconds => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Hours;

impl_new!(Hours; hours);
impl_into! {
    Hours: 0 = value * R2H;
    Hours: 0 * H2R;
    Hours => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct HoursMinutes;

impl_new!(HoursMinutes; hours, minutes);
impl_into! {
    HoursMinutes: Left = value * R2H;
    HoursMinutes: Left * H2R;
    HoursMinutes => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct HoursMinutesSeconds;

impl_new!(HoursMinutesSeconds; hours, minutes, seconds);
impl_into! {
    HoursMinutesSeconds: Left = value * R2H;
    HoursMinutesSeconds: Left * R2H;
    HoursMinutesSeconds => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Minutes;

impl_new!(Minutes; minutes);
impl_into! {
    Minutes: 0 = value * R2M;
    Minutes: 0 * M2R;
    Minutes => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct MinutesSeconds;

impl_new!(MinutesSeconds; minutes, seconds);
impl_into! {
    MinutesSeconds: Left = value * R2M;
    MinutesSeconds: Left * M2R;
    MinutesSeconds => Radians;
}


#[derive(AngleMapper, Clone, Copy, Debug)]
pub struct Seconds;

impl_new!(Seconds; seconds);
impl_into! {
    Seconds: 0 = value * R2S;
    Seconds: 0 * S2R;
    Seconds => Radians;
}


#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform;

    use super::*;

    const EPS: f64 = 1e-8;
    const ITERATIONS: i32 = 200;

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