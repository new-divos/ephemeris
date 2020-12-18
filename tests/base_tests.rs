mod common;

#[macro_use]
extern crate approx;
#[macro_use]
extern crate lazy_static;

use std::f64::consts::{FRAC_PI_6, FRAC_PI_4, FRAC_PI_3, FRAC_PI_2};

use ephemeris::base;
use ephemeris::base::{Fractional, Modulo};

#[test]
fn fractional_test() {
    let f = 3.7_f64;
    let g = 3.0_f64;
    let h = -3.7_f64;

    assert_relative_eq!(f.fractional(), 0.7);
    assert_relative_eq!(g.fractional(), 0.0);
    assert_relative_eq!(h.fractional(), 0.3);

    let k = 3.7_f32;
    let l = 3.0_f32;
    let m = -3.7_f32;

    assert_relative_eq!(k.fractional(), 0.7);
    assert_relative_eq!(l.fractional(), 0.0);
    assert_relative_eq!(m.fractional(), 0.3);
}

#[test]
fn modulo_test() {
    let n = 360_f64;

    let m = n + 45_f64;
    assert_relative_eq!(m.modulo(n), 45.0);
    let m = -45_f64;
    assert_relative_eq!(m.modulo(n), n + m);
    let m = 10.0 * n + 45_f64;
    assert_relative_eq!(m.modulo(n), 45.0);

    let n = 360_f32;

    let m = n + 45_f32;
    assert_relative_eq!(m.modulo(n), 45.0);
    let m = -45_f32;
    assert_relative_eq!(m.modulo(n), n + m);
    let m = 10.0 * n + 45_f32;
    assert_relative_eq!(m.modulo(n), 45.0);
}

#[test]
fn pertpair_test() {
    const FRAC_1_2: f64 = 0.5;

    lazy_static! {
        static ref FRAC_SQRT2_2: f64 = 2_f64.sqrt() / 2.0;
        static ref FRAC_SQRT3_2: f64 = 3_f64.sqrt() / 2.0;
    }

    let p = base::PertPair::default();
    assert_eq!(p.c(), 1.0);
    assert_eq!(p.s(), 0.0);

    let p6 = base::PertPair::from(FRAC_PI_6);
    assert_relative_eq!(p6.c(), *FRAC_SQRT3_2);
    assert_relative_eq!(p6.s(), FRAC_1_2);

    let p4 = base::PertPair::from(FRAC_PI_4);
    assert_relative_eq!(p4.c(), *FRAC_SQRT2_2);
    assert_relative_eq!(p4.s(), *FRAC_SQRT2_2);

    let p3 = base::PertPair::from(FRAC_PI_3);
    assert_relative_eq!(p3.c(), FRAC_1_2);
    assert_relative_eq!(p3.s(), *FRAC_SQRT3_2);

    let p2 = base::PertPair::from(FRAC_PI_2);
    assert_relative_eq!(p2.c(), 0.0);
    assert_relative_eq!(p2.s(), 1.0);

    let ps = p6 + p3;
    assert_relative_eq!(ps.c(), p2.c(), epsilon = common::EPS);
    assert_relative_eq!(ps.s(), p2.s(), epsilon = common::EPS);

    let ps = p4 + p4;
    assert_relative_eq!(ps.c(), p2.c(), epsilon = common::EPS);
    assert_relative_eq!(ps.s(), p2.s(), epsilon = common::EPS);

    let mut pm = p6;
    pm += p3;
    assert_relative_eq!(pm.c(), p2.c(), epsilon = common::EPS);
    assert_relative_eq!(pm.s(), p2.s(), epsilon = common::EPS);
}
