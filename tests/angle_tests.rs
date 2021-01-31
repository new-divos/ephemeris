#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use std::convert;

use ephem::base::angle::*;
use ephem::base::consts::{PI2, DEG, RAD};


fn to_short(value: f64) -> (i32, f64) {
    let t = value.abs();
    let value1 = t.floor();
    let mut value2 = 60.0 * (t - value1);
    let mut value1 = value1 as i32;

    if value < 0.0 {
        if value1 == 0 {
            value2 = -value2;
        } else {
            value1 = -value1;
        }
    }

    (value1, value2)
}


fn from_short(value1: i32, value2: f64) -> f64 {
    let mut value = (value1.abs() as f64) + value2.abs() / 60.0;
    if value1 < 0 || (value1 == 0 && value2 < 0.0) {
        value = -value;
    }

    value
}


fn from_ishort(value1: i32, value2: i32) -> i32 {
    let mut value = 60 * value1.abs() + value2.abs();
    if value1 < 0 || (value1 == 0 && value2 < 0) {
        value = -value;
    }

    value
}


fn to_long(value: f64) -> (i32, i32, f64) {
    let t = value.abs();
    let value1 = t.floor();
    let m = 60.0 * (t - value1);
    let value2 = m.floor();
    let mut value3 = 60.0 * (m - value2);
    let mut value2 = value2 as i32;
    let mut value1 = value1 as i32;

    if value < 0.0 {
        if value1 == 0 {
            if value2 == 0 {
                value3 = -value3;
            } else {
                value2 = -value2;
            }
        } else {
            value1 = -value1;
        }
    }

    (value1, value2, value3)
}


fn from_long(value1: i32, value2: i32, value3: f64) -> f64 {
    let mut value = (value1.abs() as f64) + ((value2.abs() as f64)
        + value3.abs() / 60.0) / 60.0;
    if value1 < 0 || (value1 == 0 && (value2 < 0 || (value2 == 0 && value3 < 0.0))) {
        value = -value;
    }

    value
}


fn test_value<T>(angle: Angle, value: f64)
where T: convert::Into<f64>,
      Angle: convert::Into<T>
{
    let raw: T = angle.into();
    let test_value: f64 = raw.into();
    assert_relative_eq!(test_value, value, epsilon = common::EPS);
}


fn test_short<T>(angle: Angle, value1: i32, value2: f64)
where T: convert::Into<(i32, f64)>,
      Angle: convert::Into<T>
{
    let raw: T = angle.into();
    let (test_value1, test_value2) = raw.into();
    assert_eq!(test_value1, value1);
    assert_relative_eq!(test_value2, value2, epsilon = common::EPS);
}


fn test_long<T>(angle: Angle, value1: i32, value2: i32, value3: f64)
where T: convert::Into<(i32, i32, f64)>,
      Angle: convert::Into<T>
{
    let raw: T = angle.into();
    let (test_value1, test_value2, test_value3) = raw.into();
    assert_eq!(test_value1, value1);
    assert_eq!(test_value2, value2);
    assert_relative_eq!(test_value3, value3, epsilon = common::EPS);
}


#[test]
fn into_rad_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-PI2, PI2);

    for _ in 0..common::ITERATIONS {
        let radians = rng.sample(band);

        let a_rev = Angle::from_r(radians / PI2);
        let mut v: f64 = a_rev.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let degrees = DEG * radians;

        let a_ad = Angle::from_ad(degrees);
        v = a_ad.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_degrees, angle_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angle_degrees, angle_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_adm,
            angle_degrees,
            angle_minutes
        );
        v = a_adm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_adms,
            angle_degrees,
            angle_minutes,
            angle_seconds
        );
        v = a_adms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        v = a_am.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        test_short::<AngleArcMinutesSeconds>(
            a_ams,
            angle_minutes,
            angle_seconds
        );
        v = a_ams.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_as = Angle::from_as(seconds);
        v = a_as.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let hours = degrees / 15.0;

        let a_th = Angle::from_th(hours);
        v = a_th.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_hours, angle_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        test_short::<AngleTimeHoursMinutes>(
            a_thm,
            angle_hours,
            angle_minutes
        );
        v = a_thm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        test_long::<AngleTimeHoursMinutesSeconds>(
            a_thms,
            angle_hours,
            angle_minutes,
            angle_seconds
        );
        v = a_thms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        v = a_tm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        test_short::<AngleTimeMinutesSeconds>(
            a_tms,
            angle_minutes,
            angle_seconds
        );
        v = a_ams.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        v = a_ts.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);
    }
}


#[test]
fn into_r_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-2.0_f64, 2.0_f64);

    for _ in 0..common::ITERATIONS {
        let revolutions = rng.sample(band);

        let a_rad = Angle::from(PI2 * revolutions);
        test_value::<AngleRevolutions>(a_rad, revolutions);

        let degrees = 360.0 * revolutions;

        let a_ad = Angle::from_ad(degrees);
        test_value::<AngleRevolutions>(a_ad, revolutions);

        let (angle_degrees, angle_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angle_degrees, angle_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_adm,
            angle_degrees,
            angle_minutes
        );
        test_value::<AngleRevolutions>(a_adm, revolutions);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_adms,
            angle_degrees,
            angle_minutes,
            angle_seconds
        );
        test_value::<AngleRevolutions>(a_adms, revolutions);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        test_value::<AngleRevolutions>(a_am, revolutions);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        test_short::<AngleArcMinutesSeconds>(
            a_ams,
            angle_minutes,
            angle_seconds
        );
        test_value::<AngleRevolutions>(a_ams, revolutions);

        let seconds = 60.0 * minutes;

        let a_as = Angle::from_as(seconds);
        test_value::<AngleRevolutions>(a_as, revolutions);

        let hours = 24.0 * revolutions;

        let a_th = Angle::from_th(hours);
        test_value::<AngleRevolutions>(a_th, revolutions);

        let (angle_hours, angle_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        test_short::<AngleTimeHoursMinutes>(
            a_thm,
            angle_hours,
            angle_minutes
        );
        test_value::<AngleRevolutions>(a_thm, revolutions);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        test_long::<AngleTimeHoursMinutesSeconds>(
            a_thms,
            angle_hours,
            angle_minutes,
            angle_seconds
        );
        test_value::<AngleRevolutions>(a_thms, revolutions);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        test_value::<AngleRevolutions>(a_tm, revolutions);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        test_short::<AngleTimeMinutesSeconds>(
            a_tms,
            angle_minutes,
            angle_seconds
        );
        test_value::<AngleRevolutions>(a_tms, revolutions);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        test_value::<AngleRevolutions>(a_ts, revolutions);
    }
}


#[test]
fn into_ad_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-360.0_f64, 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(band);

        let a_rad = Angle::from(RAD * degrees);
        test_value::<AngleArcDegrees>(a_rad, degrees);

        let a_rev = Angle::from_r(degrees / 360.0);
        test_value::<AngleArcDegrees>(a_rev, degrees);

        let (angle_degrees, angle_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angle_degrees, angle_minutes);
        test_value::<AngleArcDegrees>(a_adm, degrees);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        test_value::<AngleArcDegrees>(a_adms, degrees);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        test_value::<AngleArcDegrees>(a_am, degrees);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        test_value::<AngleArcDegrees>(a_ams, degrees);

        let seconds = 60.0 * minutes;

        let a_as = Angle::from_as(seconds);
        test_value::<AngleArcDegrees>(a_as, degrees);

        let hours = degrees / 15.0;

        let a_th = Angle::from_th(hours);
        test_value::<AngleArcDegrees>(a_th, degrees);

        let (angle_hours, angle_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        test_value::<AngleArcDegrees>(a_thm, degrees);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        test_value::<AngleArcDegrees>(a_thms, degrees);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        test_value::<AngleArcDegrees>(a_tm, degrees);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        test_value::<AngleArcDegrees>(a_tms, degrees);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        test_value::<AngleArcDegrees>(a_ts, degrees);
    }
}


#[test]
fn into_adm_test() {
    let mut rng = thread_rng();
    let degrees_band = Uniform::new(-360i32, 360i32);
    let minutes_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(degrees_band);
        let minutes = rng.sample(minutes_band);

        let total_degrees = from_short(degrees, minutes);

        let a_rad = Angle::from(RAD * total_degrees);
        test_short::<AngleArcDegreesMinutes>(
            a_rad,
            degrees,
            minutes
        );

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test_short::<AngleArcDegreesMinutes>(
            a_rev,
            degrees,
            minutes
        );

        let a_ad = Angle::from_ad(total_degrees);
        test_short::<AngleArcDegreesMinutes>(
            a_ad,
            degrees,
            minutes
        );

        let angle_minutes = minutes.floor();
        let angle_seconds = 60.0 * (minutes - angle_minutes);
        let angle_minutes = angle_minutes as i32;

        let a_adms = Angle::from_adms(degrees, angle_minutes, angle_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_adms,
            degrees,
            minutes
        );

        let total_minutes = 60.0 * total_degrees;

        let a_am = Angle::from_am(total_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_am,
            degrees,
            minutes
        );

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_ams,
            degrees,
            minutes
        );

        let total_seconds = 60.0 * total_minutes;

        let a_as = Angle::from_as(total_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_as,
            degrees,
            minutes
        );

        let total_hours = total_degrees / 15.0;

        let a_th = Angle::from_th(total_hours);
        test_short::<AngleArcDegreesMinutes>(
            a_th,
            degrees,
            minutes
        );

        let (angle_hours, angle_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_thm,
            degrees,
            minutes
        );

        let (angle_hours, angle_minutes, angle_seconds) = to_long(total_hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_thms,
            degrees,
            minutes
        );

        let total_minutes = 60.0 * total_hours;

        let a_tm = Angle::from_tm(total_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_tm,
            degrees,
            minutes
        );

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_tms,
            degrees,
            minutes
        );

        let total_seconds = 60.0 * total_minutes;

        let a_ts = Angle::from_ts(total_seconds);
        test_short::<AngleArcDegreesMinutes>(
            a_ts,
            degrees,
            minutes
        );
    }
}


#[test]
fn into_adms_test() {
    let mut rng = thread_rng();
    let degrees_band = Uniform::new(-360i32, 360i32);
    let minutes_band = Uniform::new(0i32, 60i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(degrees_band);
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let total_degrees = from_long(degrees, minutes, seconds);

        let a_rad = Angle::from(total_degrees * RAD);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_rad,
            degrees,
            minutes,
            seconds
        );

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_rev,
            degrees,
            minutes,
            seconds
        );

        let a_ad = Angle::from_ad(total_degrees);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_ad,
            degrees,
            minutes,
            seconds
        );

        let angle_minutes = from_short(minutes, seconds);
        let a_adm = Angle::from_adm(degrees, angle_minutes);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_adm,
            degrees,
            minutes,
            seconds
        );

        let total_minutes = total_degrees * 60.0;

        let a_am = Angle::from_am(total_minutes);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_am,
            degrees,
            minutes,
            seconds
        );

        let angle_minutes = from_ishort(degrees, minutes);
        let a_ams = Angle::from_ams(angle_minutes, seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_ams,
            degrees,
            minutes,
            seconds
        );

        let total_seconds = total_minutes * 60.0;

        let a_as = Angle::from_as(total_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_as,
            degrees,
            minutes,
            seconds
        );

        let total_hours = total_degrees / 15.0;

        let a_th = Angle::from_th(total_hours);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_th,
            degrees,
            minutes,
            seconds
        );

        let (angle_hours, angle_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_thm,
            degrees,
            minutes,
            seconds
        );

        let (angle_hours, angle_minutes, angle_seconds) = to_long(total_hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_thms,
            degrees,
            minutes,
            seconds
        );

        let total_minutes = 60.0 * total_hours;

        let a_tm = Angle::from_tm(total_minutes);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_tm,
            degrees,
            minutes,
            seconds
        );

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_tms,
            degrees,
            minutes,
            seconds
        );

        let total_seconds = 60.0 * total_minutes;

        let a_ts = Angle::from_ts(total_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_ts,
            degrees,
            minutes,
            seconds
        );
    }
}
