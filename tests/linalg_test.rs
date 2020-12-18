use ephemeris::base::linalg;
use ephemeris::base::linalg::Convert;

#[test]
fn create_cartesian_vec3d_test() {
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

