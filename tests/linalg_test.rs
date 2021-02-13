#![allow(dead_code)]

mod common;

#[macro_use]
extern crate approx;

use std::f64::consts::{PI, FRAC_PI_2};
use rand::Rng;

use ephem::vec3d;
use ephem::base::linalg;
use ephem::base::consts::PI2;


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
    assert_eq!(zero.x(), 0.0);
    assert_eq!(zero.y(), 0.0);
    assert_eq!(zero.z(), 0.0);

    let (x, y, z) = zero.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 0.0);

    let ux = linalg::Vec3D::<linalg::Cartesian>::unit_x();
    assert_eq!(ux.x(), 1.0);
    assert_eq!(ux.y(), 0.0);
    assert_eq!(ux.z(), 0.0);

    let (x, y, z) = ux.into();
    assert_eq!(x, 1.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 0.0);

    let uy = linalg::Vec3D::<linalg::Cartesian>::unit_y();
    assert_eq!(uy.x(), 0.0);
    assert_eq!(uy.y(), 1.0);
    assert_eq!(uy.z(), 0.0);

    let (x, y, z) = uy.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 1.0);
    assert_eq!(z, 0.0);

    let uz = linalg::Vec3D::<linalg::Cartesian>::unit_z();
    assert_eq!(uz.x(), 0.0);
    assert_eq!(uz.y(), 0.0);
    assert_eq!(uz.z(), 1.0);

    let (x, y, z) = uz.into();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(z, 1.0);

    let v = linalg::Vec3D::<linalg::Cartesian>::new(1.0, 2.0, 3.0);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);

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
        assert_eq!(v.x(), x);
        assert_eq!(v.y(), y);
        assert_eq!(v.z(), z);

        let c: linalg::Vec3D<linalg::Cylindrical> = v.into();
        let t: linalg::Vec3D<linalg::Cartesian> = c.into();
        assert_relative_eq!(t.x(), x, epsilon=common::EPS);
        assert_relative_eq!(t.y(), y, epsilon=common::EPS);
        assert_relative_eq!(t.z(), z, epsilon=common::EPS);

        let s: linalg::Vec3D<linalg::Spherical> = v.into();
        let t: linalg::Vec3D<linalg::Cartesian> = s.into();
        assert_relative_eq!(t.x(), x, epsilon=common::EPS);
        assert_relative_eq!(t.y(), y, epsilon=common::EPS);
        assert_relative_eq!(t.z(), z, epsilon=common::EPS);
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
        assert_eq!(v.rho(), rho);
        assert_eq!(v.phi(), phi);
        assert_eq!(v.z(), z);

        let c: linalg::Vec3D<linalg::Cartesian> = v.into();
        let t: linalg::Vec3D<linalg::Cylindrical> = c.into();
        assert_relative_eq!(t.rho(), rho, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.z(), z, epsilon=common::EPS);

        let s: linalg::Vec3D<linalg::Spherical> = v.into();
        let t: linalg::Vec3D<linalg::Cylindrical> = s.into();
        assert_relative_eq!(t.rho(), rho, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.z(), z, epsilon=common::EPS);
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
        assert_eq!(v.r(), r);
        assert_eq!(v.phi(), phi);
        assert_eq!(v.theta(), theta);

        let c: linalg::Vec3D<linalg::Cartesian> = v.into();
        let t: linalg::Vec3D<linalg::Spherical> = c.into();
        assert_relative_eq!(t.r(), r, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.theta(), theta, epsilon=common::EPS);

        let y: linalg::Vec3D<linalg::Cylindrical> = v.into();
        let t: linalg::Vec3D<linalg::Spherical> = y.into();
        assert_relative_eq!(t.r(), r, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.theta(), theta, epsilon=common::EPS);
    }

    for _ in 0..common::ITERATIONS {
        let phi = PI2 * rng.gen::<f64>();
        let theta = PI * rng.gen::<f64>() - FRAC_PI_2;

        let u = linalg::Vec3D::<linalg::Spherical>::unit(phi, theta);
        assert_eq!(u.r(), 1.0);
        assert_eq!(u.phi(), phi);
        assert_eq!(u.theta(), theta);

        let c: linalg::Vec3D<linalg::Cartesian> = u.into();
        let t: linalg::Vec3D<linalg::Spherical> = c.into();
        assert_relative_eq!(t.r(), 1.0, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.theta(), theta, epsilon=common::EPS);

        let y: linalg::Vec3D<linalg::Cylindrical> = u.into();
        let t: linalg::Vec3D<linalg::Spherical> = y.into();
        assert_relative_eq!(t.r(), 1.0, epsilon=common::EPS);
        assert_relative_eq!(t.phi(), phi, epsilon=common::EPS);
        assert_relative_eq!(t.theta(), theta, epsilon=common::EPS);
    }
}

#[test]
fn vec3d_operation_neg_test() {
    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v = new_random_vector3d(&mut rng);
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
        let v1 = new_random_vector3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vector3d(&mut rng);
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
        let v1 = new_random_vector3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vector3d(&mut rng);
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
        let v = new_random_vector3d(&mut rng);
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

    let v = linalg::Vector3D::from_c(1.0, 2.0, 3.0);
    let r = v / 0.0;
    assert!(r.is_err());

    for _ in 0..common::ITERATIONS {
        let v = new_random_vector3d(&mut rng);
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
        let v1 = new_random_vector3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vector3d(&mut rng);
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
    let v = linalg::Vector3D::unit_x().cross(linalg::Vector3D::unit_y());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vector3D::unit_z();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vector3D::unit_y().cross(linalg::Vector3D::unit_x());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vector3D::unit_z();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vector3D::unit_y().cross(linalg::Vector3D::unit_z());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vector3D::unit_x();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vector3D::unit_z().cross(linalg::Vector3D::unit_y());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vector3D::unit_x();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vector3D::unit_z().cross(linalg::Vector3D::unit_x());
    let result: linalg::CartesianVec3D = v.into();

    let r = linalg::Vector3D::unit_y();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let v = linalg::Vector3D::unit_x().cross(linalg::Vector3D::unit_z());
    let result: linalg::CartesianVec3D = v.into();

    let r = -linalg::Vector3D::unit_y();
    let raw: linalg::CartesianVec3D = r.into();
    assert_eq!(result, raw);

    let mut rng = rand::thread_rng();

    for _ in 0..common::ITERATIONS {
        let v1 = new_random_vector3d(&mut rng);
        let raw1: linalg::CartesianVec3D = v1.into();

        let v2 = new_random_vector3d(&mut rng);
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
