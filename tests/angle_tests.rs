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
                value2 = -value2;
            } else {
                value3 = -value3;
            }
        } else {
            value1 = -value1;
        }
    }

    (value1, value2, value3)
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

        // Отдельно проверить для значений degrees
        // -0.6065354189331629
        // -0.014147194375659922
        // -0.5497574740735445
        // На этих значениях тесты не проходят.

        let (angle_degrees, angle_minutes, angle_seconds) = to_long(degrees);
        let a_adms = Angle::from_adms(angle_degrees, angle_minutes, angle_seconds);
        let angle: AngleArcDegreesMinutesSeconds = a_adms.into();
        let (test_degrees, test_minutes, test_second) = angle.raw();
        assert_eq!(test_degrees, angle_degrees);
        assert_eq!(test_minutes as i32, angle_minutes);
        assert_relative_eq!(test_second, angle_seconds, epsilon = common::EPS);
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
    }
}
