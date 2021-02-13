#[macro_export]
macro_rules! vec3d {
    (x) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cartesian>::unit_x()
    );
    (y) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cartesian>::unit_y()
    );
    (z) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cartesian>::unit_z()
    );
    (x=$x:expr, y=$y:expr, z=$z:expr) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cartesian>::new($x, $y, $z)
    );
    (rho=$rho:expr, phi=$phi:expr, z=$z:expr) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cylindrical>::new(
            $rho, $phi, $z)
    );
    (phi=$phi:expr, theta=$theta:expr) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Spherical>::unit(
            $phi, $theta)
    );
    (r=$r:expr, phi=$phi:expr, theta=$theta:expr) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Spherical>::new(
            $r, $phi, $theta)
    );
    ($x:expr, $y:expr, $z:expr) => (
        $crate::base::linalg::Vec3D::<$crate::base::linalg::Cartesian>::new($x, $y, $z)
    );
}