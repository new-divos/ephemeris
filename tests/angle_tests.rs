#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use std::convert;

use ephem::base::angle::*;
use ephem::base::consts::{PI2, R2D, D2R, R2AS};


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

        let degrees = R2D * radians;

        let a_ad = Angle::from_ad(degrees);
        v = a_ad.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_adm,
            angular_degrees,
            angular_minutes
        );
        v = a_adm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_adms,
            angular_degrees,
            angular_minutes,
            angular_seconds
        );
        v = a_adms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        v = a_am.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test_short::<AngleArcMinutesSeconds>(
            a_ams,
            angular_minutes,
            angular_seconds
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

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test_short::<AngleTimeHoursMinutes>(
            a_thm,
            angular_hours,
            angular_minutes
        );
        v = a_thm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test_long::<AngleTimeHoursMinutesSeconds>(
            a_thms,
            angular_hours,
            angular_minutes,
            angular_seconds
        );
        v = a_thms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        v = a_tm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test_short::<AngleTimeMinutesSeconds>(
            a_tms,
            angular_minutes,
            angular_seconds
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
        let test = |angle: Angle| test_value::<AngleRevolutions>(
            angle, revolutions);

        let a_rad = Angle::from(PI2 * revolutions);
        test(a_rad);

        let degrees = 360.0 * revolutions;
        let a_ad = Angle::from_ad(degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test_short::<AngleArcDegreesMinutes>(
            a_adm,
            angular_degrees,
            angular_minutes
        );
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test_long::<AngleArcDegreesMinutesSeconds>(
            a_adms,
            angular_degrees,
            angular_minutes,
            angular_seconds
        );
        test(a_adms);

        let minutes = 60.0 * degrees;
        let a_am = Angle::from_am(minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test_short::<AngleArcMinutesSeconds>(
            a_ams,
            angular_minutes,
            angular_seconds
        );
        test(a_ams);

        let seconds = 60.0 * minutes;
        let a_as = Angle::from_as(seconds);
        test(a_as);

        let hours = 24.0 * revolutions;
        let a_th = Angle::from_th(hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test_short::<AngleTimeHoursMinutes>(
            a_thm,
            angular_hours,
            angular_minutes
        );
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test_long::<AngleTimeHoursMinutesSeconds>(
            a_thms,
            angular_hours,
            angular_minutes,
            angular_seconds
        );
        test(a_thms);

        let minutes = 60.0 * hours;
        let a_tm = Angle::from_tm(minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test_short::<AngleTimeMinutesSeconds>(
            a_tms,
            angular_minutes,
            angular_seconds
        );
        test(a_tms);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        test(a_ts);
    }
}


#[test]
fn into_ad_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-360.0_f64, 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleArcDegrees>(
            angle, degrees);

        let a_rad = Angle::from(D2R * degrees);
        test(a_rad);

        let a_rev = Angle::from_r(degrees / 360.0);
        test(a_rev);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let minutes = 60.0 * degrees;
        let a_am = Angle::from_am(minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(minutes * 60.0);
        test(a_as);

        let hours = degrees / 15.0;
        let a_th = Angle::from_th(hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let minutes = 60.0 * hours;
        let a_tm = Angle::from_tm(minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(minutes * 60.0);
        test(a_ts);
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

        let test = |angle: Angle| test_short::<AngleArcDegreesMinutes>(
            angle, degrees, minutes);

        let total_degrees = from_short(degrees, minutes);

        let a_rad = Angle::from(D2R * total_degrees);
        test(a_rad);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let angular_minutes = minutes.floor();
        let angular_seconds = 60.0 * (minutes - angular_minutes);
        let angular_minutes = angular_minutes as i32;

        let a_adms = Angle::from_adms(degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let total_minutes = 60.0 * total_degrees;
        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(total_minutes * 60.0);
        test(a_as);

        let total_hours = total_degrees / 15.0;
        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let total_minutes = 60.0 * total_hours;
        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(total_minutes * 60.0);
        test(a_ts);
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

        let test = |angle: Angle| test_long::<AngleArcDegreesMinutesSeconds>(
            angle, degrees, minutes, seconds);

        let total_degrees = from_long(degrees, minutes, seconds);

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let angular_minutes = from_short(minutes, seconds);
        let a_adm = Angle::from_adm(degrees, angular_minutes);
        test(a_adm);

        let total_minutes = total_degrees * 60.0;
        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let angular_minutes = from_ishort(degrees, minutes);
        let a_ams = Angle::from_ams(angular_minutes, seconds);
        test(a_ams);

        let a_as = Angle::from_as(total_minutes * 60.0);
        test(a_as);

        let total_hours = total_degrees / 15.0;

        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let total_minutes = 60.0 * total_hours;
        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(total_minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_am_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-60.0 * 360.0_f64, 60.0 * 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleArcMinutes>(
            angle, minutes);

        let degrees = minutes / 60.0;

        let a_rad = Angle::from(degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(minutes * 60.0);
        test(a_as);

        let hours = degrees / 15.0;
        let a_th = Angle::from_th(hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let angular_minutes = minutes / 15.0;
        let a_tm = Angle::from_tm(angular_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(angular_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(minutes * 4.0);
        test(a_ts);
    }
}


#[test]
fn into_ams_test() {
    let mut rng = thread_rng();
    let minutes_band = Uniform::new(-60 * 360i32, 60 * 360i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let test = |angle: Angle| test_short::<AngleArcMinutesSeconds>(
            angle, minutes, seconds);

        let total_minutes = from_short(minutes, seconds);
        let total_degrees = total_minutes / 60.0;

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(total_degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(total_degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let a_as = Angle::from_as(60.0 * total_minutes);
        test(a_as);

        let total_hours = total_degrees / 15.0;
        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let total_minutes = total_minutes / 15.0;
        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(60.0 * total_minutes);
        test(a_ts);
    }
}


#[test]
fn into_as_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-3600.0 * 360.0_f64, 3600.0 * 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let seconds = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleArcSeconds>(
            angle, seconds);

        let minutes = seconds / 60.0;
        let degrees = minutes / 60.0;

        let a_rad = Angle::from(seconds / R2AS);
        test(a_rad);

        let a_rev = Angle::from_r(degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let a_am = Angle::from_am(minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let hours = degrees / 15.0;
        let a_th = Angle::from_th(hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let minutes = minutes / 15.0;
        let a_tm = Angle::from_tm(minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(seconds / 15.0);
        test(a_ts);
    }
}


#[test]
fn into_th_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-24.0_f64, 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleTimeHours>(
            angle, hours);

        let degrees = hours * 15.0;
        let a_rad = Angle::from(degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(hours / 24.0);
        test(a_rev);

        let a_ad = Angle::from_ad(degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let minutes = degrees * 60.0;
        let a_am = Angle::from_am(minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(minutes * 60.0);
        test(a_as);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_adm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_adm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let minutes = hours * 60.0;
        let a_tm = Angle::from_tm(minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_thm_test() {
    let mut rng = thread_rng();
    let hours_band = Uniform::new(-24, 24);
    let minutes_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(hours_band);
        let minutes = rng.sample(minutes_band);

        let test = |angle: Angle| test_short::<AngleTimeHoursMinutes>(
            angle, hours, minutes);

        let total_hours = from_short(hours, minutes);
        let total_degrees = total_hours * 15.0;

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_hours / 24.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(total_degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(total_degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let total_minutes = total_degrees * 60.0;
        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(total_minutes * 60.0);
        test(a_as);

        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let total_minutes = total_hours * 60.0;
        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(total_minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_thms_test() {
    let mut rng = thread_rng();
    let hours_band = Uniform::new(-24, 24);
    let minutes_band = Uniform::new(0i32, 60i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(hours_band);
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let test = |angle: Angle| test_long::<AngleTimeHoursMinutesSeconds>(
            angle, hours, minutes, seconds);

        let total_hours = from_long(hours, minutes, seconds);
        let total_degrees = total_hours * 15.0;

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_hours / 24.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(total_degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(total_degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let total_minutes = total_degrees * 60.0;
        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(total_minutes * 60.0);
        test(a_as);

        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let total_minutes = total_hours * 60.0;
        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(total_minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_tm_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-60.0 * 24.0_f64, 60.0 * 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleTimeMinutes>(
            angle, minutes);

        let total_degrees = minutes / 4.0;

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(total_degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(total_degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let total_minutes = minutes * 15.0;
        let a_am = Angle::from_am(total_minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(total_minutes * 60.0);
        test(a_as);

        let total_hours = minutes / 60.0;
        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);

        let a_ts = Angle::from_ts(minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_tms_test() {
    let mut rng = thread_rng();
    let minutes_band = Uniform::new(-24 * 60i32, 24 * 60i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let test = |angle: Angle| test_short::<AngleTimeMinutesSeconds>(
            angle, minutes, seconds);

        let total_minutes = from_short(minutes, seconds);
        let arc_minutes = total_minutes * 15.0;
        let total_degrees = arc_minutes / 60.0;

        let a_rad = Angle::from(total_degrees * D2R);
        test(a_rad);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(total_degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(total_degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(total_degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let a_am = Angle::from_am(arc_minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(arc_minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(arc_minutes * 60.0);
        test(a_as);

        let total_hours = total_minutes / 60.0;
        let a_th = Angle::from_th(total_hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) =
            to_long(total_hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let a_tm = Angle::from_tm(total_minutes);
        test(a_tm);

        let a_ts = Angle::from_ts(total_minutes * 60.0);
        test(a_ts);
    }
}


#[test]
fn into_ts_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-3600.0 * 24.0_f64, 3600.0 * 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let seconds = rng.sample(band);
        let test = |angle: Angle| test_value::<AngleTimeSeconds>(
            angle, seconds);

        let arc_seconds = seconds * 15.0;
        let minutes = arc_seconds / 60.0;
        let degrees = minutes / 60.0;

        let a_rad = Angle::from(arc_seconds / R2AS);
        test(a_rad);

        let a_rev = Angle::from_r(degrees / 360.0);
        test(a_rev);

        let a_ad = Angle::from_ad(degrees);
        test(a_ad);

        let (angular_degrees, angular_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angular_degrees, angular_minutes);
        test(a_adm);

        let (angular_degrees, angular_minutes, angular_seconds) =
            to_long(degrees);
        let a_adms = Angle::from_adms(angular_degrees, angular_minutes, angular_seconds);
        test(a_adms);

        let a_am = Angle::from_am(minutes);
        test(a_am);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angular_minutes, angular_seconds);
        test(a_ams);

        let a_as = Angle::from_as(arc_seconds);
        test(a_as);

        let minutes = seconds / 60.0;
        let hours = minutes / 60.0;

        let a_th = Angle::from_th(hours);
        test(a_th);

        let (angular_hours, angular_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angular_hours, angular_minutes);
        test(a_thm);

        let (angular_hours, angular_minutes, angular_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angular_hours, angular_minutes, angular_seconds);
        test(a_thms);

        let a_tm = Angle::from_tm(minutes);
        test(a_tm);

        let (angular_minutes, angular_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angular_minutes, angular_seconds);
        test(a_tms);
    }
}