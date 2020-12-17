use ephemeris::base::linalg;
use ephemeris::base::linalg::Convert;

#[test]
fn create_cartesian_vec3d() {
    let mut v1 = linalg::Vec3D::new_cartesian(1.0, 2.0, 3.0);
    let raw: linalg::CartesianVec3D = v1.convert();
    assert_eq!(raw, linalg::CartesianVec3D { x: 1.0, y: 2.0, z: 3.0 });

    let opt: Option<&mut linalg::CartesianVec3D> = v1.unwrap();
    if let Some(raw) = opt {
        raw.x = 5.0;
        raw.y = 6.0;
        raw.z = 7.0;
    } else {
        panic!("Cannot unwrap cartesian vector");
    }

    let raw: linalg::CartesianVec3D = v1.convert();
    assert_eq!(raw, linalg::CartesianVec3D { x: 5.0, y: 6.0, z: 7.0 })
}

#[test]
fn mat3d_mul() {
    let a = linalg::Mat3D(
        [
            [5.0, 8.0, -4.0],
            [6.0, 9.0, -5.0],
            [4.0, 7.0, -3.0]
        ]
    );
    let b = linalg::Mat3D(
        [
            [3.0,  2.0, 5.0],
            [4.0, -1.0, 3.0],
            [9.0,  6.0, 5.0]
        ]
    );
    let c = linalg::Mat3D(
        [
            [11.0, -22.0, 29.0],
            [ 9.0, -27.0, 32.0],
            [13.0, -17.0, 26.0]
        ]
    );

    assert_eq!(a * b, c);

    let mut d = a;
    d *= b;

    assert_eq!(d, c);

    let a = linalg::Mat3D(
        [
            [ 1.0,  3.0,  4.0],
            [-1.0,  7.0,  9.0],
            [ 4.0,  3.0,  8.0]
        ]
    );
    let b = linalg::Mat3D(
        [
            [9.0,  1.0,  4.0],
            [1.0, -9.0,  5.0],
            [1.0,  2.0,  9.0]
        ]
    );
    let c = linalg::Mat3D(
        [
            [16.0, -18.0,  55.0],
            [ 7.0, -46.0, 112.0],
            [47.0,  -7.0, 103.0]
        ]
    );

    assert_eq!(a * b, c);

    let mut d = a;
    d *= b;

    assert_eq!(d, c);
}

#[test]
fn mat3d_det() {
    let a = linalg::Mat3D(
        [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ]
    );
    assert_eq!(a.det(), 0.0);

    let b = linalg::Mat3D(
        [
            [1.0,  4.0, 8.0],
            [8.0, -3.0, 4.0],
            [4.0,  8.0, 8.0]
        ]
    );
    assert_eq!(b.det(), 360.0);

    let c = linalg::Mat3D(
        [
            [1.0, 2.0,  3.0],
            [4.0, 5.0,  6.0],
            [7.0, 8.0, -9.0]
        ]
    );
    assert_eq!(c.det(), 54.0);
}
