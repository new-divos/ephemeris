#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use ephem::base::angle::*;
use ephem::base::consts::MULT_2_PI;


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


#[test]
fn into_r_test() {
    let mut rng = thread_rng();
    let band = Uniform::new(-2.0_f64, 2.0_f64);

    for _ in 0..common::ITERATIONS {
        let revolutions = rng.sample(band);

        let a_rad = Angle::from(MULT_2_PI * revolutions);
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

        let hours = 24.0 * revolutions;

        let a_th = Angle::from_th(hours);
        v = a_th.into();
        assert_relative_eq!(v.value(), revolutions);
    }
}
