#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

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
        let angle: AngleArcDegreesMinutes = a_adm.into();
        let (test_degrees, test_minutes) = angle.raw();
        assert_eq!(test_degrees, angle_degrees);
        assert_relative_eq!(test_minutes, angle_minutes, epsilon = common::EPS);
        v = a_adm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        let angle: AngleArcDegreesMinutesSeconds = a_adms.into();
        let (test_degrees, test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_degrees, angle_degrees);
        assert_eq!(test_minutes as i32, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_adms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        v = a_am.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        let angle: AngleArcMinutesSeconds = a_ams.into();
        let (test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_minutes, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
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
        let angle: AngleTimeHoursMinutes = a_thm.into();
        let (test_hours, test_minutes) = angle.raw();
        assert_eq!(test_hours, angle_hours);
        assert_relative_eq!(test_minutes, angle_minutes, epsilon = common::EPS);
        v = a_thm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        let angle: AngleTimeHoursMinutesSeconds = a_thms.into();
        let (test_hours, test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_hours, angle_hours);
        assert_eq!(test_minutes as i32, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_thms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        v = a_tm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        let angle: AngleTimeMinutesSeconds = a_tms.into();
        let (test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_minutes, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
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
        let mut v: AngleRevolutions = a_rad.into();
        assert_relative_eq!(v.value(), revolutions);

        let degrees = 360.0 * revolutions;

        let a_ad = Angle::from_ad(degrees);
        v = a_ad.into();
        assert_relative_eq!(v.value(), revolutions);

        let (angle_degrees, angle_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angle_degrees, angle_minutes);
        let angle: AngleArcDegreesMinutes = a_adm.into();
        let (test_degrees, test_minutes) = angle.raw();
        assert_eq!(test_degrees, angle_degrees);
        assert_relative_eq!(test_minutes, angle_minutes, epsilon = common::EPS);
        v = a_adm.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        let angle: AngleArcDegreesMinutesSeconds = a_adms.into();
        let (test_degrees, test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_degrees, angle_degrees);
        assert_eq!(test_minutes as i32, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_adms.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        v = a_am.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        let angle: AngleArcMinutesSeconds = a_ams.into();
        let (test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_minutes, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_ams.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_as = Angle::from_as(seconds);
        v = a_as.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let hours = 24.0 * revolutions;

        let a_th = Angle::from_th(hours);
        v = a_th.into();
        assert_relative_eq!(v.value(), revolutions);

        let (angle_hours, angle_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        let angle: AngleTimeHoursMinutes = a_thm.into();
        let (test_hours, test_minutes) = angle.raw();
        assert_eq!(test_hours, angle_hours);
        assert_relative_eq!(test_minutes, angle_minutes, epsilon = common::EPS);
        v = a_thm.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        let angle: AngleTimeHoursMinutesSeconds = a_thms.into();
        let (test_hours, test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_hours, angle_hours);
        assert_eq!(test_minutes as i32, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_thms.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        v = a_tm.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        let angle: AngleTimeMinutesSeconds = a_tms.into();
        let (test_minutes, test_seconds) = angle.raw();
        assert_eq!(test_minutes, angle_minutes);
        assert_relative_eq!(test_seconds, angle_seconds, epsilon = common::EPS);
        v = a_ams.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        v = a_ts.into();
        assert_relative_eq!(v.value(), revolutions, epsilon = common::EPS);
    }
}


#[test]
fn into_ad_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-360.0_f64, 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(band);

        let a_rad = Angle::from(RAD * degrees);
        let mut v: AngleArcDegrees = a_rad.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let a_rev = Angle::from_r(degrees / 360.0);
        v = a_rev.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_degrees, angle_minutes) = to_short(degrees);
        let a_adm = Angle::from_adm(angle_degrees, angle_minutes);
        v = a_adm.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        v = a_adms.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let minutes = 60.0 * degrees;

        let a_am = Angle::from_am(minutes);
        v = a_am.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        v = a_ams.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_as = Angle::from_as(seconds);
        v = a_as.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let hours = degrees / 15.0;

        let a_th = Angle::from_th(hours);
        v = a_th.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_hours, angle_minutes) = to_short(hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        v = a_thm.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        v = a_thms.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let minutes = 60.0 * hours;

        let a_tm = Angle::from_tm(minutes);
        v = a_tm.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        v = a_tms.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);

        let seconds = 60.0 * minutes;

        let a_ts = Angle::from_ts(seconds);
        v = a_ts.into();
        assert_relative_eq!(v.value(), degrees, epsilon = common::EPS);
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
        let mut v: AngleArcDegreesMinutes = a_rad.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        v = a_rev.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let a_ad = Angle::from_ad(total_degrees);
        v = a_ad.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let angle_minutes = minutes.floor();
        let angle_seconds = 60.0 * (minutes - angle_minutes);
        let angle_minutes = angle_minutes as i32;

        let a_adms = Angle::from_adms(degrees, angle_minutes, angle_seconds);
        v = a_adms.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let total_minutes = 60.0 * total_degrees;

        let a_am = Angle::from_am(total_minutes);
        v = a_am.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_ams = Angle::from_ams(angle_minutes, angle_seconds);
        v = a_ams.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let total_seconds = 60.0 * total_minutes;

        let a_as = Angle::from_as(total_seconds);
        v = a_as.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let total_hours = total_degrees / 15.0;

        let a_th = Angle::from_th(total_hours);
        v = a_th.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let (angle_hours, angle_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        v = a_thm.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(total_hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        v = a_thms.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let total_minutes = 60.0 * total_hours;

        let a_tm = Angle::from_tm(total_minutes);
        v = a_tm.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        v = a_tms.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);

        let total_seconds = 60.0 * total_minutes;

        let a_ts = Angle::from_ts(total_seconds);
        v = a_ts.into();
        let (test_degrees, test_minutes) = v.raw();
        assert_eq!(degrees, test_degrees);
        assert_relative_eq!(minutes, test_minutes, epsilon = common::EPS);
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
        let mut v: AngleArcDegreesMinutesSeconds = a_rad.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let a_rev = Angle::from_r(total_degrees / 360.0);
        v = a_rev.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let a_ad = Angle::from_ad(total_degrees);
        v = a_ad.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let angle_minutes = from_short(minutes, seconds);
        let a_adm = Angle::from_adm(degrees, angle_minutes);
        v = a_adm.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let total_minutes = total_degrees * 60.0;

        let a_am = Angle::from_am(total_minutes);
        v = a_am.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let angle_minutes = from_ishort(degrees, minutes);
        let a_ams = Angle::from_ams(angle_minutes, seconds);
        v = a_ams.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let total_seconds = total_minutes * 60.0;

        let a_as = Angle::from_as(total_seconds);
        v = a_as.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let total_hours = total_degrees / 15.0;

        let a_th = Angle::from_th(total_hours);
        v = a_th.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let (angle_hours, angle_minutes) = to_short(total_hours);
        let a_thm = Angle::from_thm(angle_hours, angle_minutes);
        v = a_thm.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let (angle_hours, angle_minutes, angle_seconds) = to_long(total_hours);
        let a_thms = Angle::from_thms(angle_hours, angle_minutes, angle_seconds);
        v = a_thms.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let total_minutes = 60.0 * total_hours;

        let a_tm = Angle::from_tm(total_minutes);
        v = a_tm.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let (angle_minutes, angle_seconds) = to_short(total_minutes);
        let a_tms = Angle::from_tms(angle_minutes, angle_seconds);
        v = a_tms.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);

        let total_seconds = 60.0 * total_minutes;

        let a_ts = Angle::from_ts(total_seconds);
        v = a_ts.into();
        let (test_degrees, test_minutes, test_seconds) = v.raw();
        assert_eq!(test_degrees, degrees);
        assert_eq!(test_minutes as i32, minutes);
        assert_relative_eq!(test_seconds, seconds, epsilon = common::EPS);
    }
}
