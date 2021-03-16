#![allow(dead_code)]

mod common;

#[macro_use] extern crate approx;
extern crate serde_json;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

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


#[test]
fn angle_new_test() {
    let mut rng = thread_rng();

    let rad_band = Uniform::new(-PI2, PI2);
    let rev_band = Uniform::new(-2.0_f64, 2.0_f64);
    let deg_band = Uniform::new(-360.0_f64, 360.0_f64);
    let hr_band = Uniform::new(-24.0_f64, 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let rs = rng.sample(rad_band);
        let a_rad = Angle::<Radians>::from(rs);
        let rd: f64 = a_rad.into();
        assert_eq!(rs, rd);

        let rvs = rng.sample(rev_band);
        let a_rev = Angle::<Revolutions>::new(rvs);
        assert_eq!(a_rev.revolutions(), rvs);

        let ds = rng.sample(deg_band);
        let a_d = Angle::<Degrees>::new(ds);
        assert_eq!(a_d.degrees(), ds);

        let (deg, amin) = to_short(ds);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(deg, amin);
        assert_eq!(a_dam.degrees(), deg);
        assert_relative_eq!(a_dam.arc_minutes(), amin, epsilon = common::EPS);

        let (deg, amin, asec) = to_long(ds);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(deg, amin, asec);
        assert_eq!(a_dams.degrees(), deg);
        assert_eq!(a_dams.arc_minutes(), amin);
        assert_relative_eq!(a_dams.arc_seconds(), asec, epsilon = common::EPS);

        let ams = ds * 60.0;
        let a_am = Angle::<ArcMinutes>::new(ams);
        assert_eq!(a_am.arc_minutes(), ams);

        let (amin, asec) = to_short(ams);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(amin, asec);
        assert_eq!(a_ams.arc_minutes(), amin);
        assert_relative_eq!(a_ams.arc_seconds(), asec, epsilon = common::EPS);

        let ascs = ams * 60.0;
        let a_as = Angle::<ArcSeconds>::new(ascs);
        assert_eq!(a_as.arc_seconds(), ascs);

        let hrs = rng.sample(hr_band);
        let a_h = Angle::<Hours>::new(hrs);
        assert_eq!(a_h.hours(), hrs);

        let (hr, min) = to_short(hrs);
        let a_hm = Angle::<HoursMinutes>::new(hr, min);
        assert_eq!(a_hm.hours(), hr);
        assert_relative_eq!(a_hm.minutes(), min, epsilon = common::EPS);

        let (hr, min, sec) = to_long(hrs);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(hr, min, sec);
        assert_eq!(a_hms.hours(), hr);
        assert_eq!(a_hms.minutes(), min);
        assert_relative_eq!(a_hms.seconds(), sec, epsilon = common::EPS);

        let ms = hrs * 60.0;
        let a_m = Angle::<Minutes>::new(ms);
        assert_eq!(a_m.minutes(), ms);

        let (min, sec) = to_short(ms);
        let a_ms = Angle::<MinutesSeconds>::new(min, sec);
        assert_eq!(a_ms.minutes(), min);
        assert_relative_eq!(a_ms.seconds(), sec, epsilon = common::EPS);

        let scs = ms * 60.0;
        let a_s = Angle::<Seconds>::new(scs);
        assert_eq!(a_s.seconds(), scs);
    }
}


#[test]
fn angle_radians_test() {
    let a = Angle::<Radians>::from(PI2);
    let r: Angle<Revolutions> = a.into();
    assert_relative_eq!(r.revolutions(), 1.0, epsilon=common::EPS);

    let mut rng = thread_rng();
    let band = Uniform::new(-PI2, PI2);

    for _ in 0..common::ITERATIONS {
        let radians = rng.sample(band);

        let a_rev = Angle::<Revolutions>::new(radians / PI2);
        let mut v: f64 = a_rev.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_rev = Angle::<Revolutions>::from(radians);
        v = a_rev.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let degrees = radians * R2D;
        let a_ad = Angle::<Degrees>::new(degrees);
        v = a_ad.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_ad = Angle::<Degrees>::from(radians);
        v = a_ad.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (deg, arcmin) = to_short(degrees);
        let a_adm = Angle::<DegreesArcMinutes>::new(deg, arcmin);
        assert_eq!(a_adm.degrees(), deg);
        assert_relative_eq!(a_adm.arc_minutes(), arcmin, epsilon = common::EPS);
        v = a_adm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_adm = Angle::<DegreesArcMinutes>::from(radians);
        v = a_adm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (deg, arcmin, arcsec) = to_long(degrees);
        let a_adms = Angle::<DegreesArcMinutesSeconds>::new(deg, arcmin, arcsec);
        assert_eq!(a_adms.degrees(), deg);
        assert_eq!(a_adms.arc_minutes(), arcmin);
        assert_relative_eq!(a_adms.arc_seconds(), arcsec, epsilon = common::EPS);
        v = a_adms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_adms = Angle::<DegreesArcMinutesSeconds>::from(radians);
        v = a_adms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let arc_minutes = degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        v = a_am.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_am = Angle::<ArcMinutes>::from(radians);
        v = a_am.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (arcmin, arcsec) = to_short(arc_minutes);
        let a_ams = Angle::<ArcMinutesSeconds>::new(arcmin, arcsec);
        assert_eq!(a_ams.arc_minutes(), arcmin);
        assert_relative_eq!(a_adms.arc_seconds(), arcsec, epsilon = common::EPS);
        v = a_ams.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_ams = Angle::<ArcMinutesSeconds>::from(radians);
        v = a_ams.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        v = a_as.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_as = Angle::<ArcSeconds>::from(radians);
        v = a_as.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let hours = degrees / 15.0;
        let a_th = Angle::<Hours>::new(hours);
        v = a_th.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_th = Angle::<Hours>::from(radians);
        v = a_th.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (hr, min) = to_short(hours);
        let a_thm = Angle::<HoursMinutes>::new(hr, min);
        assert_eq!(a_thm.hours(), hr);
        assert_relative_eq!(a_thm.minutes(), min, epsilon = common::EPS);
        v = a_thm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_thm = Angle::<HoursMinutes>::from(radians);
        v = a_thm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (hr, min, sec) = to_long(hours);
        let a_thms =
            Angle::<HoursMinutesSeconds>::new(hr, min, sec);
        assert_eq!(a_thms.hours(), hr);
        assert_eq!(a_thms.minutes(), min);
        assert_relative_eq!(a_thms.seconds(), sec, epsilon = common::EPS);
        v = a_thms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_thms = Angle::<HoursMinutesSeconds>::from(radians);
        v = a_thms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let minutes = hours * 60.0;
        let a_tm = Angle::<Minutes>::new(minutes);
        v = a_tm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_tm = Angle::<Minutes>::from(radians);
        v = a_tm.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let (min, sec) = to_short(minutes);
        let a_tms = Angle::<MinutesSeconds>::new(min, sec);
        assert_eq!(a_tms.minutes(), min);
        assert_relative_eq!(a_tms.seconds(), sec, epsilon = common::EPS);
        v = a_tms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_tms = Angle::<MinutesSeconds>::from(radians);
        v = a_tms.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_ts = Angle::<Seconds>::new(minutes * 60.0);
        v = a_ts.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);

        let a_ts = Angle::<Seconds>::from(radians);
        v = a_ts.into();
        assert_relative_eq!(v, radians, epsilon = common::EPS);
    }
}


#[test]
fn angle_revolutions_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-2.0_f64, 2.0_f64);

    for _ in 0..common::ITERATIONS {
        let revolutions = rng.sample(band);
        let test = |angle: Angle<Revolutions>| {
            assert_relative_eq!(angle.revolutions(), revolutions, epsilon = common::EPS);
        };

        let a_rad = Angle::<Revolutions>::from(PI2 * revolutions);
        test(a_rad.into());

        let degrees = revolutions * 360.0;
        let a_ad = Angle::<Degrees>::new(degrees);
        test(a_ad.into());

        let (deg, arcmin) = to_short(degrees);
        let a_adm =
            Angle::<DegreesArcMinutes>::new(deg, arcmin);
        assert_eq!(a_adm.degrees(), deg);
        assert_relative_eq!(a_adm.arc_minutes(), arcmin, epsilon = common::EPS);
        test(a_adm.into());

        let (deg, arcmin, arcsec) = to_long(degrees);
        let a_adms =
            Angle::<DegreesArcMinutesSeconds>::new(deg, arcmin, arcsec);
        assert_eq!(a_adms.degrees(), deg);
        assert_eq!(a_adms.arc_minutes(), arcmin);
        assert_relative_eq!(a_adms.arc_seconds(), arcsec, epsilon = common::EPS);
        test(a_adms.into());

        let arc_minutes = degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (arcmin, arcsec) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(arcmin, arcsec);
        assert_eq!(a_ams.arc_minutes(), arcmin);
        assert_relative_eq!(a_ams.arc_seconds(), arcsec, epsilon = common::EPS);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        test(a_as.into());

        let hours = revolutions * 24.0;
        let a_th = Angle::<Hours>::new(hours);
        test(a_th.into());

        let (hr, min) = to_short(hours);
        let a_thm = Angle::<HoursMinutes>::new(hr, min);
        assert_eq!(a_thm.hours(), hr);
        assert_relative_eq!(a_thm.minutes(), min, epsilon=common::EPS);
        test(a_thm.into());

        let (hr, min, sec) = to_long(hours);
        let a_thms =
            Angle::<HoursMinutesSeconds>::new(hr, min, sec);
        assert_eq!(a_thms.hours(), hr);
        assert_eq!(a_thms.minutes(), min);
        assert_relative_eq!(a_thms.seconds(), sec, epsilon=common::EPS);
        test(a_thms.into());

        let minutes = hours * 60.0;
        let a_tm = Angle::<Minutes>::new(minutes);
        test(a_tm.into());

        let (min, sec) = to_short(minutes);
        let a_tms = Angle::<MinutesSeconds>::new(min, sec);
        assert_eq!(a_tms.minutes(), min);
        assert_relative_eq!(a_thms.seconds(), sec, epsilon=common::EPS);
        test(a_tms.into());

        let a_ts = Angle::<Seconds>::new(minutes * 60.0);
        test(a_ts.into());
    }
}


#[test]
fn into_degrees_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-360.0_f64, 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(band);
        let test = |angle: Angle::<Degrees>| {
            assert_relative_eq!(angle.degrees(), degrees, epsilon=common::EPS);
        };

        let a_rad = Angle::<Radians>::from(degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(degrees / 360.0);
        test(a_rev.into());

        let (deg, arcmin) = to_short(degrees);
        let a_adm =
            Angle::<DegreesArcMinutes>::new(deg, arcmin);
        test(a_adm.into());

        let (deg, arcmin, arcsec) = to_long(degrees);
        let a_adms =
            Angle::<DegreesArcMinutesSeconds>::new(deg, arcmin, arcsec);
        test(a_adms.into());

        let arc_minutes = degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (arcmin, arcsec) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(arcmin, arcsec);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        test(a_as.into());

        let hours = degrees / 15.0;
        let a_th = Angle::<Hours>::new(hours);
        test(a_th.into());

        let (hr, min) = to_short(hours);
        let a_thm = Angle::<HoursMinutes>::new(hr, min);
        test(a_thm.into());

        let (hr, min, sec) = to_long(hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(hr, min, sec);
        test(a_hms.into());

        let minutes = hours * 60.0;
        let a_tm = Angle::<Minutes>::new(minutes);
        test(a_tm.into());

        let (min, sec) = to_short(minutes);
        let a_tms = Angle::<MinutesSeconds>::new(min, sec);
        test(a_tms.into());

        let a_ts = Angle::<Seconds>::new(minutes * 60.0);
        test(a_ts.into());
    }
}


#[test]
fn into_degrees_arc_minutes_test() {
    let mut rng = thread_rng();
    let degrees_band = Uniform::new(-360i32, 360i32);
    let arc_minutes_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(degrees_band);
        let arc_minutes = rng.sample(arc_minutes_band);

        let test = |angle: Angle<DegreesArcMinutes>| {
            assert_eq!(angle.degrees(), degrees);
            assert_relative_eq!(angle.arc_minutes(), arc_minutes, epsilon = common::EPS);
        };

        let total_degrees = from_short(degrees, arc_minutes);

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let test_arc_minutes = arc_minutes.floor();
        let test_arc_seconds = 60.0 * (arc_minutes - test_arc_minutes);
        let test_arc_minutes = test_arc_minutes as i32;

        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(degrees, test_arc_minutes, test_arc_seconds);
        test(a_dams.into());

        let total_arc_minutes = total_degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(total_arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let total_hours = total_degrees / 15.0;
        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) = to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(test_hours, test_minutes, test_seconds);
        test(a_hms.into());

        let total_minutes = total_hours * 60.0;
        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(total_minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_degreese_arc_minutes_seconds_test() {
    let mut rng = thread_rng();
    let degrees_band = Uniform::new(-360i32, 360i32);
    let arc_minutes_band = Uniform::new(0i32, 60i32);
    let arc_seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let degrees = rng.sample(degrees_band);
        let arc_minutes = rng.sample(arc_minutes_band);
        let arc_seconds = rng.sample(arc_seconds_band);

        let test = |angle: Angle<DegreesArcMinutesSeconds>| {
            assert_eq!(angle.degrees(), degrees);
            assert_eq!(angle.arc_minutes(), arc_minutes);
            assert_relative_eq!(angle.arc_seconds(), arc_seconds, epsilon = common::EPS);
        };

        let total_degrees = from_long(degrees, arc_minutes, arc_seconds);

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let test_arc_minutes = from_short(arc_minutes, arc_seconds);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(degrees, test_arc_minutes);
        test(a_dam.into());

        let total_arc_minutes = total_degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let test_arc_minutes = from_ishort(degrees, arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let total_hours = total_degrees / 15.0;

        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) =
            to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(test_hours, test_minutes, test_seconds);
        test(a_hms.into());

        let total_minutes = total_hours * 60.0;
        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(total_minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_arc_minutes_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-60.0 * 360.0_f64, 60.0 * 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let arc_minutes = rng.sample(band);
        let test = |angle: Angle<ArcMinutes>|
            assert_relative_eq!(angle.arc_minutes(), arc_minutes, epsilon = common::EPS);

        let degrees = arc_minutes / 60.0;

        let a_rad = Angle::<Radians>::from(degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(degrees / 360.0);
        test(a_rev.into());

        let a_ad = Angle::<Degrees>::new(degrees);
        test(a_ad.into());

        let (test_degrees, test_arc_minutes) = to_short(degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        test(a_as.into());

        let hours = degrees / 15.0;
        let a_h = Angle::<Hours>::new(hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) = to_long(hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(test_hours, test_minutes, test_seconds);
        test(a_hms.into());

        let minutes = arc_minutes / 15.0;
        let a_m = Angle::<Minutes>::new(minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_arc_minutes_seconds_test() {
    let mut rng = thread_rng();
    let arc_minutes_band = Uniform::new(-60 * 360i32, 60 * 360i32);
    let arc_seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let arc_minutes = rng.sample(arc_minutes_band);
        let arc_seconds = rng.sample(arc_seconds_band);

        let test = |angle: Angle<ArcMinutesSeconds>| {
            assert_eq!(angle.arc_minutes(), arc_minutes);
            assert_relative_eq!(angle.arc_seconds(), arc_seconds, epsilon = common::EPS);
        };

        let total_arc_minutes = from_short(arc_minutes, arc_seconds);
        let total_degrees = total_arc_minutes / 60.0;

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(total_degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(total_degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let total_hours = total_degrees / 15.0;
        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) =
            to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(
                test_hours,
                test_minutes,
                test_seconds
            );
        test(a_hms.into());

        let total_minutes = total_arc_minutes / 15.0;
        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(total_minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_arc_seconds_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-3600.0 * 360.0_f64, 3600.0 * 360.0_f64);

    for _ in 0..common::ITERATIONS {
        let arc_seconds = rng.sample(band);
        let test = |angle: Angle<ArcSeconds>| {
            assert_relative_eq!(angle.arc_seconds(), arc_seconds, epsilon = common::EPS);
        };

        let arc_minutes = arc_seconds / 60.0;
        let degrees = arc_minutes / 60.0;

        let a_rad = Angle::<Radians>::from(arc_seconds / R2AS);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let hours = degrees / 15.0;
        let a_h = Angle::<Hours>::new(hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) = to_long(hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(test_hours, test_minutes, test_seconds);
        test(a_hms.into());

        let minutes = arc_minutes / 15.0;
        let a_m = Angle::<Minutes>::new(minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(arc_seconds / 15.0);
        test(a_s.into());
    }
}


#[test]
fn into_hours_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-24.0_f64, 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(band);
        let test = |angle: Angle<Hours>| {
            assert_relative_eq!(angle.hours(), hours, epsilon = common::EPS);
        };

        let degrees = hours * 15.0;
        let a_rad = Angle::<Radians>::from(degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(hours / 24.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(degrees);
        let a_dam = Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let arc_minutes = degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        test(a_as.into());

        let (test_hours, test_minutes) = to_short(hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) = to_long(hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(
                test_hours,
                test_minutes,
                test_seconds
            );
        test(a_hms.into());

        let minutes = hours * 60.0;
        let a_m = Angle::<Minutes>::new(minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(minutes);
        let a_ms =
            Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_hours_minutes_test() {
    let mut rng = thread_rng();
    let hours_band = Uniform::new(-24, 24);
    let minutes_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(hours_band);
        let minutes = rng.sample(minutes_band);

        let test = |angle: Angle<HoursMinutes>| {
            assert_eq!(angle.hours(), hours);
            assert_relative_eq!(angle.minutes(), minutes, epsilon = common::EPS);
        };

        let total_hours = from_short(hours, minutes);
        let total_degrees = total_hours * 15.0;

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_hours / 24.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(total_degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(total_degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let total_arc_minutes = total_degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(total_arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes, test_seconds) =
            to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(test_hours, test_minutes, test_seconds);
        test(a_hms.into());

        let total_minutes = total_hours * 60.0;
        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(total_minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_hours_minutes_seconds_test() {
    let mut rng = thread_rng();
    let hours_band = Uniform::new(-24, 24);
    let minutes_band = Uniform::new(0i32, 60i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let hours = rng.sample(hours_band);
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let test = |angle: Angle<HoursMinutesSeconds>| {
            assert_eq!(angle.hours(), hours);
            assert_eq!(angle.minutes(), minutes);
            assert_relative_eq!(angle.seconds(), seconds, epsilon = common::EPS);
        };

        let total_hours = from_long(hours, minutes, seconds);
        let total_degrees = total_hours * 15.0;

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_hours / 24.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(total_degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(total_degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let total_arc_minutes = total_degrees * 60.0;
        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(total_arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let total_minutes = total_hours * 60.0;
        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(total_minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_minutes_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-60.0 * 24.0_f64, 60.0 * 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(band);
        let test = |angle: Angle<Minutes>| {
            assert_relative_eq!(angle.minutes(), minutes, epsilon = common::EPS);
        };

        let total_degrees = minutes / 4.0;

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(total_degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(total_degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let total_arc_minutes = minutes * 15.0;
        let a_am = Angle::<ArcMinutes>::new(total_arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(total_arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(total_arc_minutes * 60.0);
        test(a_as.into());

        let total_hours = minutes / 60.0;
        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) =
            to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(
                test_hours,
                test_minutes,
                test_seconds
            );
        test(a_hms.into());

        let (test_minutes, test_seconds) = to_short(minutes);
        let a_ms = Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());

        let a_s = Angle::<Seconds>::new(minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_minutes_seconds_test() {
    let mut rng = thread_rng();
    let minutes_band = Uniform::new(-24 * 60i32, 24 * 60i32);
    let seconds_band = Uniform::new(0.0_f64, 60.0_f64);

    for _ in 0..common::ITERATIONS {
        let minutes = rng.sample(minutes_band);
        let seconds = rng.sample(seconds_band);

        let test = |angle: Angle<MinutesSeconds>| {
            assert_eq!(angle.minutes(), minutes);
            assert_relative_eq!(angle.seconds(), seconds, epsilon = common::EPS);
        };

        let total_minutes = from_short(minutes, seconds);
        let arc_minutes = total_minutes * 15.0;
        let total_degrees = arc_minutes / 60.0;

        let a_rad = Angle::<Radians>::from(total_degrees * D2R);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(total_degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(total_degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(total_degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(total_degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_minutes * 60.0);
        test(a_as.into());

        let total_hours = total_minutes / 60.0;
        let a_h = Angle::<Hours>::new(total_hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(total_hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) =
            to_long(total_hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(
                test_hours,
                test_minutes,
                test_seconds
            );
        test(a_hms.into());

        let a_m = Angle::<Minutes>::new(total_minutes);
        test(a_m.into());

        let a_s = Angle::<Seconds>::new(total_minutes * 60.0);
        test(a_s.into());
    }
}


#[test]
fn into_seconds_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-3600.0 * 24.0_f64, 3600.0 * 24.0_f64);

    for _ in 0..common::ITERATIONS {
        let seconds = rng.sample(band);
        let test = |angle: Angle<Seconds>| {
            assert_relative_eq!(angle.seconds(), seconds, epsilon = common::EPS);
        };

        let arc_seconds = seconds * 15.0;
        let arc_minutes = arc_seconds / 60.0;
        let degrees = arc_minutes / 60.0;

        let a_rad = Angle::<Radians>::from(arc_seconds / R2AS);
        test(a_rad.into());

        let a_rev = Angle::<Revolutions>::new(degrees / 360.0);
        test(a_rev.into());

        let a_d = Angle::<Degrees>::new(degrees);
        test(a_d.into());

        let (test_degrees, test_arc_minutes) = to_short(degrees);
        let a_dam =
            Angle::<DegreesArcMinutes>::new(test_degrees, test_arc_minutes);
        test(a_dam.into());

        let (test_degrees, test_arc_minutes, test_arc_seconds) =
            to_long(degrees);
        let a_dams =
            Angle::<DegreesArcMinutesSeconds>::new(
                test_degrees,
                test_arc_minutes,
                test_arc_seconds
            );
        test(a_dams.into());

        let a_am = Angle::<ArcMinutes>::new(arc_minutes);
        test(a_am.into());

        let (test_arc_minutes, test_arc_seconds) = to_short(arc_minutes);
        let a_ams =
            Angle::<ArcMinutesSeconds>::new(test_arc_minutes, test_arc_seconds);
        test(a_ams.into());

        let a_as = Angle::<ArcSeconds>::new(arc_seconds);
        test(a_as.into());

        let minutes = seconds / 60.0;
        let hours = minutes / 60.0;

        let a_h = Angle::<Hours>::new(hours);
        test(a_h.into());

        let (test_hours, test_minutes) = to_short(hours);
        let a_hm = Angle::<HoursMinutes>::new(test_hours, test_minutes);
        test(a_hm.into());

        let (test_hours, test_minutes, test_seconds) = to_long(hours);
        let a_hms =
            Angle::<HoursMinutesSeconds>::new(
                test_hours,
                test_minutes,
                test_seconds
            );
        test(a_hms.into());

        let a_m = Angle::<Minutes>::new(minutes);
        test(a_m.into());

        let (test_minutes, test_seconds) = to_short(minutes);
        let a_ms =
            Angle::<MinutesSeconds>::new(test_minutes, test_seconds);
        test(a_ms.into());
    }
}


#[test]
fn radians_serde_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-PI2, PI2);

    for _ in 0..common::ITERATIONS {
        let radians = rng.sample(band);

        let rad = Angle::<Radians>::from(radians);
        let data = serde_json::to_string(&rad).unwrap();

        let mut text = vec![r#"{"radians":"#];
        let r = format!("{}", radians);
        text.push(r.as_str());
        text.push("}");

        assert_eq!(data, text.join(""));

        let tested: Angle<Radians> = serde_json::from_str(data.as_str()).unwrap();
        let tested: f64 = tested.into();
        assert_relative_eq!(radians, tested, epsilon=common::EPS);
    }
}
