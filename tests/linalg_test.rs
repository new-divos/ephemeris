use ephem::base::linalg;

#[test]
fn create_cartesian_vec3d_test() {
    let z = linalg::Vec3D::zero();
    assert!(z.is_cartesian());
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
    assert!(ux.is_cartesian());
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
    assert!(uy.is_cartesian());
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
    assert!(uz.is_cartesian());
    let raw: linalg::CartesianVec3D = uz.into();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 1.0);

    let opt: Option<linalg::CartesianVec3D> = uz.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 0.0);
    assert_eq!(raw.y(), 0.0);
    assert_eq!(raw.z(), 1.0);

    let v = linalg::Vec3D::cartesian(1.0, 2.0, 3.0);
    assert!(v.is_cartesian());
    let raw: linalg::CartesianVec3D = v.into();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 2.0);
    assert_eq!(raw.z(), 3.0);

    let opt: Option<linalg::CartesianVec3D> = v.into();
    let raw = opt.unwrap();
    assert_eq!(raw.x(), 1.0);
    assert_eq!(raw.y(), 2.0);
    assert_eq!(raw.z(), 3.0);
}

