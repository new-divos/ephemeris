#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use std::f64::consts::{PI, FRAC_PI_2};
use rand::Rng;

use ephem::base::linalg;
use ephem::base::consts::PI2;


fn new_random_vec3d<R: Rng + ?Sized>(rng: &mut R) -> linalg::Vec3D {
    linalg::Vec3D::from_c(
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0,
        200.0 * rng.gen::<f64>() - 100.0
    )
}

#[test]
fn create_cartesian_vec3d_test() {
    let z = linalg::Vec3D::zero();
    assert!(z.is_c());
    let raw: linalg::CartesianVec3D = z.into();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 0.0);

    let opt: Option<linalg::CartesianVec3D> = z.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 0.0);

    let ux = linalg::Vec3D::unit_x();
    assert!(ux.is_c());
    let raw: linalg::CartesianVec3D = ux.into();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 0.0);

    let opt: Option<linalg::CartesianVec3D> = ux.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 0.0);

    let uy = linalg::Vec3D::unit_y();
    assert!(uy.is_c());
    let raw: linalg::CartesianVec3D = uy.into();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 1.0);
    assert_eq!(raw.z(), 0.0);

    let opt: Option<linalg::CartesianVec3D> = uy.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 1.0);
    assert_eq!(raw.z(), 0.0);

    let uz = linalg::Vec3D::unit_z();
    assert!(uz.is_c());
    let raw: linalg::CartesianVec3D = uz.into();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 1.0);

    let opt: Option<linalg::CartesianVec3D> = uz.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 1.0);

    let v = linalg::Vec3D::from_c(1.0, 2.0, 3.0);
    assert!(v.is_c());
    let raw: linalg::CartesianVec3D = v.into();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 2.0);
    assert_eq!(raw.z(), 3.0);

    let opt: Option<linalg::CartesianVec3D> = v.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 2.0);
    assert_eq!(raw.z(), 3.0);

    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let x = 200.0 * rng.gen::<f64>() - 100.0;
        let y = 200.0 * rng.gen::<f64>() - 100.0;
        let z = 200.0 * rng.gen::<f64>() - 100.0;

        let v = linalg::Vec3D::from_c(x, y, z);
        assert!(v.is_c());
        let raw: linalg::CartesianVec3D = v.into();
        assert_eq!(raw.x(), x);
        assert_eq!(raw.y(), y);
        assert_eq!(raw.z(), z);

        let opt: Option<linalg::CartesianVec3D> = v.into();
        let raw = opt.unwrap();
        assert_eq!(raw.x(), x);
        assert_eq!(raw.y(), y);
        assert_eq!(raw.z(), z);
    }
}

#[test]
fn create_cylindrical_vec3d_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let rho = 100.0 * rng.gen::<f64>();
        let phi = PI2 * rng.gen::<f64>();
        let z = 200.0 * rng.gen::<f64>() - 100.0;

        let c = linalg::Vec3D::from_y(rho, phi, z).unwrap();
        assert!(c.is_y());

        let raw: linalg::CylindricalVec3D = c.into();
        assert_eq!(raw.rho(), rho);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.z(), z);

        let opt: Option<linalg::CylindricalVec3D> = c.into();
        let raw = opt.unwrap();
        assert_eq!(raw.rho(), rho);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.z(), z);

        let a = c.to_c();
        let b = a.to_y();

        let tr: linalg::CylindricalVec3D = b.into();
        assert_relative_eq!(raw.rho(), tr.rho(), epsilon = common::EPS);
        assert_relative_eq!(raw.phi(), tr.phi(), epsilon = common::EPS);
        assert_relative_eq!(raw.z(), tr.z());
    }
}

#[test]
fn create_spherical_vec3d_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..common::ITERATIONS {
        let r = 100.0 * rng.gen::<f64>();
        let phi = PI2 * rng.gen::<f64>();
        let theta = PI * rng.gen::<f64>() - FRAC_PI_2;

        let s = linalg::Vec3D::from_s(r, phi, theta).unwrap();
        assert!(s.is_s());

        let raw: linalg::SphericalVec3D = s.into();
        assert_eq!(raw.r(), r);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.theta(), theta);

        let opt: Option<linalg::SphericalVec3D> = s.into();
        let raw = opt.unwrap();
        assert_eq!(raw.r(), r);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.theta(), theta);

        let a = s.to_c();
        let b = a.to_s();

        let tr: linalg::SphericalVec3D = b.into();
        assert_relative_eq!(raw.r(), tr.r(), epsilon = common::EPS);
        assert_relative_eq!(raw.phi(), tr.phi(), epsilon = common::EPS);
        assert_relative_eq!(raw.theta(), tr.theta(), epsilon = common::EPS);
    }

    for _ in 0..common::ITERATIONS {
        let phi = PI2 * rng.gen::<f64>();
        let theta = PI * rng.gen::<f64>() - FRAC_PI_2;

        let u = linalg::Vec3D::unit(phi, theta).unwrap();
        assert!(u.is_s());

        let raw: linalg::SphericalVec3D = u.into();
        assert_eq!(raw.r(), 1.0);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.theta(), theta);

        let opt: Option<linalg::SphericalVec3D> = u.into();
        let raw = opt.unwrap();
        assert_eq!(raw.r(), 1.0);
        assert_eq!(raw.phi(), phi);
        assert_eq!(raw.theta(), theta);

        let a = u.to_y();
        let b = a.to_s();

        let tr: linalg::SphericalVec3D = b.into();
        assert_relative_eq!(raw.r(), tr.r(), epsilon = common::EPS);
        assert_relative_eq!(raw.phi(), tr.phi(), epsilon = common::EPS);
        assert_relative_eq!(raw.theta(), tr.theta(), epsilon = common::EPS);
    }
}

#[test]
fn vec3d_operation_neg_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v = new_random_vec3d(&mut rng);
        let raw: linalg::CartesianVec3D = v.into();

        let r = -v;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), -raw.x());
        assert_eq!(result.y(), -raw.y());
        assert_eq!(result.z(), -raw.z());
    }
}

#[test]
fn vec3d_operation_add_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let raw2: linalg::CartesianVec3D = v2.into();

        let r = v1 + v2;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw1.x() + raw2.x());
        assert_eq!(result.y(), raw1.y() + raw2.y());
        assert_eq!(result.z(), raw1.z() + raw2.z());

        let mut r = v1;
        r += v2;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw1.x() + raw2.x());
        assert_eq!(result.y(), raw1.y() + raw2.y());
        assert_eq!(result.z(), raw1.z() + raw2.z());

        let mut s1 = v1.to_s();
        s1 += v2;
        assert!(s1.is_s());
        let result2: linalg::SphericalVec3D = s1.into();

        let s2 = r.to_s();
        assert!(s2.is_s());
        let result: linalg::SphericalVec3D = s2.into();

        assert_relative_eq!(result.r(), result2.r(), epsilon = common::EPS);
        assert_relative_eq!(result.phi(), result2.phi(), epsilon = common::EPS);
        assert_relative_eq!(result.theta(), result2.theta(), epsilon = common::EPS);

        let mut c1 = v1.to_y();
        c1 += v2;
        assert!(c1.is_y());
        let result2: linalg::CylindricalVec3D = c1.into();

        let c2 = r.to_y();
        assert!(c2.is_y());
        let result: linalg::CylindricalVec3D = c2.into();

        assert_relative_eq!(result.rho(), result2.rho(), epsilon = common::EPS);
        assert_relative_eq!(result.phi(), result2.phi(), epsilon = common::EPS);
        assert_relative_eq!(result.z(), result2.z(), epsilon = common::EPS);
    }
}

#[test]
fn vec3d_operation_sub_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let raw2: linalg::CartesianVec3D = v2.into();

        let r = v1 - v2;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw1.x() - raw2.x());
        assert_eq!(result.y(), raw1.y() - raw2.y());
        assert_eq!(result.z(), raw1.z() - raw2.z());

        let mut r = v1;
        r -= v2;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw1.x() - raw2.x());
        assert_eq!(result.y(), raw1.y() - raw2.y());
        assert_eq!(result.z(), raw1.z() - raw2.z());

        let mut s1 = v1.to_s();
        s1 -= v2;
        assert!(s1.is_s());
        let result2: linalg::SphericalVec3D = s1.into();

        let s2 = r.to_s();
        assert!(s2.is_s());
        let result: linalg::SphericalVec3D = s2.into();

        assert_relative_eq!(result.r(), result2.r(), epsilon = common::EPS);
        assert_relative_eq!(result.phi(), result2.phi(), epsilon = common::EPS);
        assert_relative_eq!(result.theta(), result2.theta(), epsilon = common::EPS);

        let mut c1 = v1.to_y();
        c1 -= v2;
        assert!(c1.is_y());
        let result: linalg::CylindricalVec3D = c1.into();

        let c2 = r.to_y();
        assert!(c2.is_y());
        let result2: linalg::CylindricalVec3D = c2.into();

        assert_relative_eq!(result2.rho(), result.rho(), epsilon = common::EPS);
        assert_relative_eq!(result2.phi(), result.phi(), epsilon = common::EPS);
        assert_relative_eq!(result2.z(), result.z(), epsilon = common::EPS);
    }
}

#[test]
fn vec3d_operation_mul_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v = new_random_vec3d(&mut rng);
        let raw: linalg::CartesianVec3D = v.into();

        let a = 200.0 * rng.gen::<f64>() - 100.0;

        let r = v * a;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw.x() * a);
        assert_eq!(result.y(), raw.y() * a);
        assert_eq!(result.z(), raw.z() * a);

        let r = a * v;
        let result: linalg::CartesianVec3D = r.into();

        assert_eq!(result.x(), raw.x() * a);
        assert_eq!(result.y(), raw.y() * a);
        assert_eq!(result.z(), raw.z() * a);

        let mut b = v;
        b *= a;
        let result2: linalg::CartesianVec3D = b.into();

        assert_eq!(result2.x(), result.x());
        assert_eq!(result2.y(), result.y());
        assert_eq!(result2.z(), result.z());

        let mut s1 = v.to_s();
        s1 *= a;
        assert!(s1.is_s());
        let result: linalg::SphericalVec3D = s1.into();

        let s2 = r.to_s();
        assert!(s2.is_s());
        let result2: linalg::SphericalVec3D = s2.into();

        assert_relative_eq!(result2.r(), result.r(), epsilon = common::EPS);
        assert_relative_eq!(result2.phi(), result.phi(), epsilon = common::EPS);
        assert_relative_eq!(result2.theta(), result.theta(), epsilon = common::EPS);

        let mut  c1 = v.to_y();
        c1 *= a;
        assert!(c1.is_y());
        let result: linalg::CylindricalVec3D = c1.into();

        let c2 = r.to_y();
        assert!(c2.is_y());
        let result2: linalg::CylindricalVec3D = c2.into();

        assert_relative_eq!(result2.rho(), result.rho(), epsilon = common::EPS);
        assert_relative_eq!(result2.phi(), result.phi(), epsilon = common::EPS);
        assert_relative_eq!(result2.z(), result.z());
    }
}

#[test]
fn vec3d_operation_div_test() {
    let mut rng = rand::thread_rng();

    let v = linalg::Vec3D::from_c(1.0, 2.0, 3.0);
    let r = v / 0.0;
    assert!(r.is_err());

    for _ in 0..common::ITERATIONS {
        let v = new_random_vec3d(&mut rng);
        let raw: linalg::CartesianVec3D = v.into();

        let a = 200.0 * rng.gen::<f64>() - 100.0;

        let r = v / a;
        match r {
            Ok(c) => {
                let result: linalg::CartesianVec3D = c.into();
                assert_relative_eq!(result.x(), raw.x() / a);
                assert_relative_eq!(result.y(), raw.y() / a);
                assert_relative_eq!(result.z(), raw.z() / a);
            },
            Err(_) => {
                assert_eq!(a, 0.0);
            }
        }
    }
}

#[test]
fn vec3d_operation_dot_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let raw2: linalg::CartesianVec3D = v2.into();

        let result = v1.dot(v2);
        assert_eq!(
            result,
            raw1.x() * raw2.x() + raw1.y() * raw2.y() + raw1.z() * raw2.z()
        );
    }
}

#[test]
fn vec3d_operation_cross_test() {
    let v = linalg::Vec3D::unit_x().cross(linalg::Vec3D::unit_y());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vec3D::unit_z();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vec3D::unit_y().cross(linalg::Vec3D::unit_x());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vec3D::unit_z();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vec3D::unit_y().cross(linalg::Vec3D::unit_z());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vec3D::unit_x();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vec3D::unit_z().cross(linalg::Vec3D::unit_y());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vec3D::unit_x();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vec3D::unit_z().cross(linalg::Vec3D::unit_x());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vec3D::unit_y();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vec3D::unit_x().cross(linalg::Vec3D::unit_z());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vec3D::unit_y();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vec3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vec3d(&mut rng);
        let raw2: linalg::CartesianVec3D = v2.into();

        let r = v1.cross(v2);
        let result: linalg::CartesianVec3D = r.into();

        let x = raw1.y() * raw2.z() - raw1.z() * raw2.y();
        let y = raw1.z() * raw2.x() - raw1.x() * raw2.z();
        let z = raw1.x() * raw2.y() - raw1.y() * raw2.x();

        assert_eq!(x, result.x());
        assert_eq!(y, result.y());
        assert_eq!(z, result.z());
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
        let v1 = new_random_vec3d(&mut rng);
        let v2 = new_random_vec3d(&mut rng);
        let v3 = new_random_vec3d(&mut rng);

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
