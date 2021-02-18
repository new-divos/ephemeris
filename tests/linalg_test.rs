#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use std::f64::consts::{PI, FRAC_PI_2};
use rand::Rng;

use ephem::vec3d;
use ephem::base::consts::PI2;
use ephem::base::error::Error;
use ephem::base::linalg;


#[test]
fn macro_test() {
    let v = vec3d![x];
    let (x, y, z) = v.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 0.0);

    let v = vec3d![y];
    let (x, y, z) = v.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 1.0);
    assert_eq!(z, 0.0);

    let v = vec3d![z];
    let (x, y, z) = v.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 1.0);

    let v = vec3d![x=1.0, y=2.0, z=3.0];
    let (x, y, z) = v.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    assert_eq!(z, 3.0);

    let v = vec3d![rho=1.0, phi=2.0, z=3.0];
    let (rho, phi, z) = v.into();
    assert_eq!(rho, 1.0);
    assert_eq!(phi, 2.0);
    assert_eq!(z, 3.0);

    let v = vec3d![phi=2.0, theta=1.0];
    let (r, phi, theta) = v.into();
    assert_eq!(r, 1.0);
    assert_eq!(phi, 2.0);
    assert_eq!(theta, 1.0);

    let v = vec3d![r=3.0, phi=2.0, theta=1.0];
    let (r, phi, theta) = v.into();
    assert_eq!(r, 3.0);
    assert_eq!(phi, 2.0);
    assert_eq!(theta, 1.0);

    let v = vec3d![1.0, 2.0, 3.0];
    let (x, y, z) = v.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    assert_eq!(z, 3.0);
}


fn new_random_vec3d<R: Rng + ?Sized>(rng: &mut R) -> linalg::Vec3D<linalg::Cartesian> {
    linalg::Vec3D::<linalg::Cartesian>::new(
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0
    )
}


fn new_random_cvec3d<R: Rng + ?Sized>(rng: &mut R) -> linalg::Vec3D<linalg::Cylindrical> {
    linalg::Vec3D::<linalg::Cylindrical>::new(
        100.0 * rng.gen::<f64>(),
        PI2 * rng.gen::<f64>(),
        200.0 * rng.gen::<f64>() - 100.0
    )
}


fn new_random_svec3d<R: Rng + ?Sized>(rng: &mut R) -> linalg::Vec3D<linalg::Spherical> {
    linalg::Vec3D::<linalg::Spherical>::new(
        100.0 * rng.gen::<f64>(),
        PI2 * rng.gen::<f64>(),
        PI * rng.gen::<f64>() - FRAC_PI_2
    )
}


fn new_random_vector3d<R: Rng + ?Sized>(rng: &mut R) -> linalg::Vector3D {
    linalg::Vector3D::from_c(
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0
    )
}


#[test]
fn create_cartesian_vec3d_test() {
    let zero = linalg::Vec3D::<linalg::Cartesian>::zero();
    assert_eq!(zero[linalg::Cartesian::X], 0.0);
    assert_eq!(zero[linalg::Cartesian::Y], 0.0);
    assert_eq!(zero[linalg::Cartesian::Z], 0.0);

    let (x, y, z) = zero.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 0.0);

    let ux = linalg::Vec3D::<linalg::Cartesian>::unit_x();
    assert_eq!(ux[linalg::Cartesian::X], 1.0);
    assert_eq!(ux[linalg::Cartesian::Y], 0.0);
    assert_eq!(ux[linalg::Cartesian::Z], 0.0);

    let (x, y, z) = ux.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 0.0);

    let uy = linalg::Vec3D::<linalg::Cartesian>::unit_y();
    assert_eq!(uy[linalg::Cartesian::X], 0.0);
    assert_eq!(uy[linalg::Cartesian::Y], 1.0);
    assert_eq!(uy[linalg::Cartesian::Z], 0.0);

    let (x, y, z) = uy.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 1.0);
    assert_eq!(z, 0.0);

    let uz = linalg::Vec3D::<linalg::Cartesian>::unit_z();
    assert_eq!(uz[linalg::Cartesian::X], 0.0);
    assert_eq!(uz[linalg::Cartesian::Y], 0.0);
    assert_eq!(uz[linalg::Cartesian::Z], 1.0);

    let (x, y, z) = uz.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 1.0);

    let v = linalg::Vec3D::<linalg::Cartesian>::new(1.0, 2.0, 3.0);
    assert_eq!(v[linalg::Cartesian::X], 1.0);
    assert_eq!(v[linalg::Cartesian::Y], 2.0);
    assert_eq!(v[linalg::Cartesian::Z], 3.0);

    let (x, y, z) = v.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    assert_eq!(z, 3.0);

    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let x = 200.0 * rng.gen::<f64>() - 100.0;
        let y = 200.0 * rng.gen::<f64>() - 100.0;
        let z = 200.0 * rng.gen::<f64>() - 100.0;

        let v = linalg::Vec3D::<linalg::Cartesian>::new(x, y, z);
        assert_eq!(v[linalg::Cartesian::X], x);
        assert_eq!(v[linalg::Cartesian::Y], y);
        assert_eq!(v[linalg::Cartesian::Z], z);

        let c: linalg::Vec3D<linalg::Cylindrical> = v.into();
        let t: linalg::Vec3D<linalg::Cartesian> = c.into();
        assert_relative_eq!(t[linalg::Cartesian::X], x, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cartesian::Y], y, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cartesian::Z], z, epsilon=common::EPS);

        let s: linalg::Vec3D<linalg::Spherical> = v.into();
        let t: linalg::Vec3D<linalg::Cartesian> = s.into();
        assert_relative_eq!(t[linalg::Cartesian::X], x, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cartesian::Y], y, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cartesian::Z], z, epsilon=common::EPS);
    }
}

#[test]
fn create_cylindrical_vec3d_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let rho = 100.0 * rng.gen::<f64>();
        let phi = PI2 * rng.gen::<f64>();
        let z = 200.0 * rng.gen::<f64>() - 100.0;

        let v = linalg::Vec3D::<linalg::Cylindrical>::new(rho, phi, z);
        assert_eq!(v[linalg::Cylindrical::Radius], rho);
        assert_eq!(v[linalg::Cylindrical::Azimuth], phi);
        assert_eq!(v[linalg::Cylindrical::Altitude], z);

        let c: linalg::Vec3D<linalg::Cartesian> = v.into();
        let t: linalg::Vec3D<linalg::Cylindrical> = c.into();
        assert_relative_eq!(t[linalg::Cylindrical::Radius], rho, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cylindrical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cylindrical::Altitude], z, epsilon=common::EPS);

        let s: linalg::Vec3D<linalg::Spherical> = v.into();
        let t: linalg::Vec3D<linalg::Cylindrical> = s.into();
        assert_relative_eq!(t[linalg::Cylindrical::Radius], rho, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cylindrical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Cylindrical::Altitude], z, epsilon=common::EPS);
    }
}

#[test]
fn create_spherical_vec3d_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let r = 100.0 * rng.gen::<f64>();
        let phi = PI2 * rng.gen::<f64>();
        let theta = PI * rng.gen::<f64>() - FRAC_PI_2;

        let v = linalg::Vec3D::<linalg::Spherical>::new(r, phi, theta);
        assert_eq!(v[linalg::Spherical::Radius], r);
        assert_eq!(v[linalg::Spherical::Azimuth], phi);
        assert_eq!(v[linalg::Spherical::Colatitude], theta);

        let c: linalg::Vec3D<linalg::Cartesian> = v.into();
        let t: linalg::Vec3D<linalg::Spherical> = c.into();
        assert_relative_eq!(t[linalg::Spherical::Radius], r, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Colatitude], theta, epsilon=common::EPS);

        let y: linalg::Vec3D<linalg::Cylindrical> = v.into();
        let t: linalg::Vec3D<linalg::Spherical> = y.into();
        assert_relative_eq!(t[linalg::Spherical::Radius], r, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Colatitude], theta, epsilon=common::EPS);
    }

    for _ in 0..common::ITERATIONS {
        let phi = PI2 * rng.gen::<f64>();
        let theta = PI * rng.gen::<f64>() - FRAC_PI_2;

        let u = linalg::Vec3D::<linalg::Spherical>::unit(phi, theta);
        assert_eq!(u[linalg::Spherical::Radius], 1.0);
        assert_eq!(u[linalg::Spherical::Azimuth], phi);
        assert_eq!(u[linalg::Spherical::Colatitude], theta);

        let c: linalg::Vec3D<linalg::Cartesian> = u.into();
        let t: linalg::Vec3D<linalg::Spherical> = c.into();
        assert_relative_eq!(t[linalg::Spherical::Radius], 1.0, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Colatitude], theta, epsilon=common::EPS);

        let y: linalg::Vec3D<linalg::Cylindrical> = u.into();
        let t: linalg::Vec3D<linalg::Spherical> = y.into();
        assert_relative_eq!(t[linalg::Spherical::Radius], 1.0, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Azimuth], phi, epsilon=common::EPS);
        assert_relative_eq!(t[linalg::Spherical::Colatitude], theta, epsilon=common::EPS);
    }
}


#[test]
fn vec3d_iter_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v = new_random_vec3d(&mut rng);
        for (i, a) in v.iter().enumerate() {
            assert_eq!(a, v[i]);
        }

        let v = new_random_cvec3d(&mut rng);
        for (i, a) in v.iter().enumerate() {
            assert_eq!(a, v[i]);
        }

        let v = new_random_svec3d(&mut rng);
        for (i, a) in v.iter().enumerate() {
            assert_eq!(a, v[i]);
        }
    }
}


#[test]
fn vec3d_operation_neg_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v = new_random_vec3d(&mut rng);
        let (x, y, z) = v.into();

        let r = -v;
        let (tx, ty, tz) = r.into();

        assert_eq!(tx, -x);
        assert_eq!(ty, -y);
        assert_eq!(tz, -z);

        let v = new_random_cvec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = -v;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let c = -c;
        let (tx, ty, tz) = c.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let v = new_random_svec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = -v;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let c = -c;
        let (tx, ty, tz) = c.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);
    }
}

#[test]
fn vec3d_operation_add_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let (x1, y1, z1) = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let (x2, y2, z2) = v2.into();

        let r = v1 + v2;
        let (x, y, z) = r.into();

        assert_eq!(x, x1 + x2);
        assert_eq!(y, y1 + y2);
        assert_eq!(z, z1 + z2);

        let mut r = v1;
        r += v2;
        let (x, y, z) = r.into();

        assert_eq!(x, x1 + x2);
        assert_eq!(y, y1 + y2);
        assert_eq!(z, z1 + z2);
    }
}

#[test]
fn vec3d_operation_sub_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let (x1, y1, z1) = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let (x2, y2, z2) = v2.into();

        let r = v1 - v2;
        let (x, y, z) = r.into();

        assert_eq!(x, x1 - x2);
        assert_eq!(y, y1 - y2);
        assert_eq!(z, z1 - z2);

        let mut r = v1;
        r -= v2;
        let (x, y, z) = r.into();

        assert_eq!(x, x1 - x2);
        assert_eq!(y, y1 - y2);
        assert_eq!(z, z1 - z2);
    }
}

#[test]
fn vec3d_operation_mul_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let k = 200.0 * rng.gen::<f64>() - 100.0;

        let v = new_random_vec3d(&mut rng);
        let (x, y, z) = v.into();

        let r = v * k;
        let (tx, ty, tz) = r.into();

        assert_eq!(tx, x * k);
        assert_eq!(ty, y * k);
        assert_eq!(tz, z * k);

        let r = k * v;
        let (tx, ty, tz) = r.into();

        assert_eq!(tx, x * k);
        assert_eq!(ty, y * k);
        assert_eq!(tz, z * k);

        let mut r = v;
        r *= k;

        assert_eq!(tx, x * k);
        assert_eq!(ty, y * k);
        assert_eq!(tz, z * k);

        let v = new_random_cvec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v * k;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = c * k;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let r = k * v;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = k * c;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let mut r = v;
        r *= k;

        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let v = new_random_svec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v * k;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = c * k;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let r = k * v;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = k * c;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let mut r = v;
        r *= k;

        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);
    }
}

#[test]
fn vec3d_operation_div_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let k = 200.0 * rng.gen::<f64>() - 100.0;
        if k == 0.0 {
            continue;
        }

        let v = new_random_vec3d(&mut rng);
        let (x, y, z) = v.into();

        let r = v / k;
        let (tx, ty, tz) = r.into();

        assert_eq!(tx, x / k);
        assert_eq!(ty, y / k);
        assert_eq!(tz, z / k);

        let mut r = v;
        r /= k;

        assert_eq!(tx, x / k);
        assert_eq!(ty, y / k);
        assert_eq!(tz, z / k);

        let v = new_random_cvec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v / k;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = c / k;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let mut r = v;
        r /= k;

        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let v = new_random_svec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v / k;
        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        let b = c / k;
        let (tx, ty, tz) = b.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);

        let mut r = v;
        r /= k;

        let t: linalg::Vec3D<linalg::Cartesian> = r.into();
        let (x, y, z) = t.into();

        assert_relative_eq!(tx, x, epsilon=common::EPS);
        assert_relative_eq!(ty, y, epsilon=common::EPS);
        assert_relative_eq!(tz, z, epsilon=common::EPS);
    }

    let v = linalg::Vec3D::<linalg::Cartesian>::filled_by(1.0);
    let r = v.try_div(0.0);
    assert!(r.is_err());

    let v = linalg::Vec3D::<linalg::Cylindrical>::filled_by(1.0);
    let r = v.try_div(0.0);
    assert!(r.is_err());

    let v = linalg::Vec3D::<linalg::Spherical>::filled_by(1.0);
    let r = v.try_div(0.0);
    assert!(r.is_err());

    for _ in 0..common::ITERATIONS {
        let k = 200.0 * rng.gen::<f64>() - 100.0;

        let v = new_random_vec3d(&mut rng);
        let (x, y, z) = v.into();

        let r = v.try_div(k);
        match r {
            Ok(t) => {
                let (tx, ty, tz) = t.into();

                assert_eq!(tx, x / k);
                assert_eq!(ty, y / k);
                assert_eq!(tz, z / k);
            },
            Err(Error::ZeroDivisionError) => {
                assert_eq!(k, 0.0);
            },
            Err(_) => {
                unreachable!();
            }
        }

        let v = new_random_cvec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v.try_div(k);
        match r {
            Ok(t) => {
                let t: linalg::Vec3D<linalg::Cartesian> = t.into();
                let (x, y, z) = t.into();

                let b = c / k;
                let (tx, ty, tz) = b.into();

                assert_relative_eq!(tx, x, epsilon=common::EPS);
                assert_relative_eq!(ty, y, epsilon=common::EPS);
                assert_relative_eq!(tz, z, epsilon=common::EPS);
            },
            Err(Error::ZeroDivisionError) => {
                assert_eq!(k, 0.0);
            },
            Err(_) => {
                unreachable!();
            }
        }

        let v = new_random_svec3d(&mut rng);
        let c: linalg::Vec3D<linalg::Cartesian> = v.into();

        let r = v.try_div(k);
        match r {
            Ok(t) => {
                let t: linalg::Vec3D<linalg::Cartesian> = t.into();
                let (x, y, z) = t.into();

                let b = c / k;
                let (tx, ty, tz) = b.into();

                assert_relative_eq!(tx, x, epsilon=common::EPS);
                assert_relative_eq!(ty, y, epsilon=common::EPS);
                assert_relative_eq!(tz, z, epsilon=common::EPS);
            },
            Err(Error::ZeroDivisionError) => {
                assert_eq!(k, 0.0);
            },
            Err(_) => {
                unreachable!();
            }
        }
    }
}

#[test]
fn vec3d_operation_dot_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let v2 = new_random_vec3d(&mut rng);

        let result = v1.dot(&v2);

        let mut value = 0.0;
        for (e1, e2) in v1.iter().zip(v2.iter()) {
            value += e1 * e2;
        }

        assert_eq!(result, value);
    }
}

#[test]
fn vec3d_operation_cross_test() {
    let i = linalg::Vec3D::<linalg::Cartesian>::unit_x();
    let j = linalg::Vec3D::<linalg::Cartesian>::unit_y();
    let k = linalg::Vec3D::<linalg::Cartesian>::unit_z();

    let r = i.cross(&j);
    assert_eq!(r, k);

    let r = j.cross(&i);
    assert_eq!(r, -k);

    let r = j.cross(&k);
    assert_eq!(r, i);

    let r = k.cross(&j);
    assert_eq!(r, -i);

    let r = k.cross(&i);
    assert_eq!(r, j);

    let r = i.cross(&k);
    assert_eq!(r, -j);

    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let (x1, y1, z1) = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let (x2, y2, z2) = v2.into();

        let r = v1.cross(&v2);
        let (x, y, z) = r.into();

        let tx = y1 * z2 - z1 * y2;
        let ty = z1 * x2 - x1 * z2;
        let tz = x1 * y2 - y1 * x2;

        assert_eq!(tx, x);
        assert_eq!(ty, y);
        assert_eq!(tz, z);
    }
}

#[test]
fn create_mat3d_test() {
    let z = linalg::Mat3D::zeros();
    for v in z.iter() {
        assert_eq!(v, 0.0);
    }

    let o = linalg::Mat3D::ones();
    for v in o.iter() {
        assert_eq!(v, 1.0);
    }

    let e = linalg::Mat3D::identity();
    for (i, v) in e.iter().enumerate() {
        let r = i / 3;
        let c = i % 3;

        if r == c {
            assert_eq!(v, 1.0);
        } else {
            assert_eq!(v, 0.0);
        }
    }

    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vector3d(&mut rng);
        let v2 = new_random_vector3d(&mut rng);
        let v3 = new_random_vector3d(&mut rng);

        let mr = linalg::Mat3D::from_rows(v1, v2, v3);
        for (idx, item) in mr.iter().enumerate() {
            let row = idx / 3;
            let col = idx % 3;

            let ref_v = match row {
                0 => &v1,
                1 => &v2,
                2 => &v3,
                _ => panic!("Illegal row number")
            };
            let raw: linalg::CartesianVec3D = (*ref_v).into();

            match col {
                0 => assert_eq!(item, raw.x()),
                1 => assert_eq!(item, raw.y()),
                2 => assert_eq!(item, raw.z()),
                _ => panic!("Illegal column number")
            };
        }

        let mc = linalg::Mat3D::from_columns(v1, v2, v3);
        for (idx, item) in mc.iter().enumerate() {
            let row = idx / 3;
            let col = idx % 3;

            let ref_v = match col {
                0 => &v1,
                1 => &v2,
                2 => &v3,
                _ => panic!("Illegal column number")
            };
            let raw: linalg::CartesianVec3D = (*ref_v).into();

            match row {
                0 => assert_eq!(item, raw.x()),
                1 => assert_eq!(item, raw.y()),
                2 => assert_eq!(item, raw.z()),
                _ => panic!("Illegal row number")
            }
        }
    }
}

#[test]
fn mat3d_rotation_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let angle = PI2 * rng.gen::<f64>();
        let tr = 1.0 + 2.0 * angle.cos();

        let rx = linalg::Mat3D::r_x(angle);
        assert_relative_eq!(rx.tr(), tr);
        assert_relative_eq!(rx.det(), 1.0);

        let irx = rx.inv().unwrap();
        for (v1, v2) in irx.iter().zip(rx.t().iter()) {
            assert_relative_eq!(v1, v2);
        }

        let ry = linalg::Mat3D::r_y(angle);
        assert_relative_eq!(ry.tr(), tr);
        assert_relative_eq!(ry.det(), 1.0);

        let iry = ry.inv().unwrap();
        for (v1, v2) in iry.iter().zip(ry.t().iter()) {
            assert_relative_eq!(v1, v2);
        }

        let rz = linalg::Mat3D::r_z(angle);
        assert_relative_eq!(rz.tr(), tr);
        assert_relative_eq!(rz.det(), 1.0);

        let irz = rz.inv().unwrap();
        for (v1, v2) in irz.iter().zip(rz.t().iter()) {
            assert_relative_eq!(v1, v2);
        }
    }
}
