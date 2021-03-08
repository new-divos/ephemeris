#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use std::convert;

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
    let a_rad = Angle::<Radians>::from(PI2);
    let r: f64 = a_rad.into();
    assert_eq!(r, PI2);

    let a_rev = Angle::<Revolutions>::new(1.0);
    assert_eq!(a_rev.revolutions(), 1.0);

    let a_d = Angle::<Degrees>::new(360.0);
    assert_eq!(a_d.degrees(), 360.0);

    let a_dam = Angle::<DegreesArcMinutes>::new(360, 0.0);
    assert_eq!(a_dam.degrees(), 360);
    assert_eq!(a_dam.arc_minutes(), 0.0);

    let a_dams =
        Angle::<DegreesArcMinutesSeconds>::new(360, 0, 0.0);
    assert_eq!(a_dams.degrees(), 360);
    assert_eq!(a_dams.arc_minutes(), 0);
    assert_eq!(a_dams.arc_seconds(), 0.0);
}