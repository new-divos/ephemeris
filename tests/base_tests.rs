#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use std::f64::consts::{FRAC_PI_6, FRAC_PI_4, FRAC_PI_3, FRAC_PI_2};

use ephem::base::*;


const FRAC_1_2: f64 = 0.5;


#[test]
fn func_test() {
    let f = 3.7_f64;
    let g = 3.0_f64;
    let h = -3.7_f64;

    assert_relative_eq!(f.frac(), 0.7);
    assert_relative_eq!(g.frac(), 0.0);
    assert_relative_eq!(h.frac(), 0.3);

    let n = 360_f64;

    let m = n + 45_f64;
    assert_relative_eq!(m.fmod(n), 45.0);
    let m = -45_f64;
    assert_relative_eq!(m.fmod(n), n + m);
    let m = 10.0 * n + 45_f64;
    assert_relative_eq!(m.fmod(n), 45.0);
}

#[test]
fn pertpair_test() {
    let frac_sqrt2_2: f64 = 2_f64.sqrt() / 2.0;
    let frac_sqrt3_2: f64 = 3_f64.sqrt() / 2.0;

    let p = PertPair::default();
    assert_eq!(p.cos(), 1.0);
    assert_eq!(p.sin(), 0.0);

    let p6 = PertPair::from(FRAC_PI_6);
    assert_relative_eq!(p6.cos(), frac_sqrt3_2);
    assert_relative_eq!(p6.sin(), FRAC_1_2);

    let p4 = PertPair::from(FRAC_PI_4);
    assert_relative_eq!(p4.cos(), frac_sqrt2_2);
    assert_relative_eq!(p4.sin(), frac_sqrt2_2);

    let p3 = PertPair::from(FRAC_PI_3);
    assert_relative_eq!(p3.cos(), FRAC_1_2);
    assert_relative_eq!(p3.sin(), frac_sqrt3_2);

    let p2 = PertPair::from(FRAC_PI_2);
    assert_relative_eq!(p2.cos(), 0.0);
    assert_relative_eq!(p2.sin(), 1.0);

    let ps = p6 + p3;
    assert_relative_eq!(ps.cos(), p2.cos(), epsilon = common::EPS);
    assert_relative_eq!(ps.sin(), p2.sin(), epsilon = common::EPS);

    let ps = p4 + p4;
    assert_relative_eq!(ps.cos(), p2.cos(), epsilon = common::EPS);
    assert_relative_eq!(ps.sin(), p2.sin(), epsilon = common::EPS);

    let mut pm = p6;
    pm += p3;
    assert_relative_eq!(pm.cos(), p2.cos(), epsilon = common::EPS);
    assert_relative_eq!(pm.sin(), p2.sin(), epsilon = common::EPS);

    let ps = p6 * 2;
    assert_relative_eq!(ps.cos(), p3.cos());
    assert_relative_eq!(ps.sin(), p3.sin());

    let ps = 2 * p6;
    assert_relative_eq!(ps.cos(), p3.cos());
    assert_relative_eq!(ps.sin(), p3.sin());

    let mut pm = p6;
    pm *= 2;
    assert_relative_eq!(pm.cos(), p3.cos());
    assert_relative_eq!(pm.sin(), p3.sin());

    let ps = p6 * 0;
    assert_eq!(ps.cos(), 1.0);
    assert_eq!(ps.sin(), 0.0);

    let ps = p6 * 1;
    assert_eq!(ps.cos(), p6.cos());
    assert_eq!(ps.sin(), p6.sin());

    let ps = p6 * -1;
    assert_eq!(ps.cos(), p6.cos());
    assert_eq!(ps.sin(), -p6.sin());

    let ps = p6 * 3;
    assert_relative_eq!(ps.cos(), p2.cos(), epsilon = common::EPS);
    assert_relative_eq!(ps.sin(), p2.sin(), epsilon = common::EPS);

    let ps = p6 * -3;
    assert_relative_eq!(ps.cos(), p2.cos(), epsilon = common::EPS);
    assert_relative_eq!(ps.sin(), -p2.sin(), epsilon = common::EPS);
}
