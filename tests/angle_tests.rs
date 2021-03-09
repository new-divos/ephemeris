#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use ephem::base::angle::*;
use ephem::base::consts::{PI2};


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