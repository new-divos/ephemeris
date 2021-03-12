use std::f64::consts::PI;

///
/// 2𝜋
///
pub const PI2: f64 = 2.0 * PI;

///
/// Degrees to radians
///
pub const D2R: f64 = PI / 180.0;

///
/// Arc minutes to radians
///
pub const AM2R: f64 = PI / (180.0 * 60.0);

///
/// Arc seconds to radians
///
pub const AS2R: f64 = PI / (180.0 * 3600.0);

///
/// Hours to radians
///
pub const H2R: f64 = PI / 12.0;

///
/// Minutes to radians
///
pub const M2R: f64 = PI / (12.0 * 60.0);

///
/// Seconds to radians
///
pub const S2R: f64 = PI / (12.0 * 3600.0);

///
/// Radians to degrees
///
pub const R2D: f64 = 180.0 / PI;

///
/// Radians to arc minutes
///
pub const R2AM: f64 = (180.0 * 60.0) / PI;

///
/// Radians to arc seconds
///
pub const R2AS: f64 = 3600.0 * 180.0 / PI;

///
/// Radians to hours
///
pub const R2H: f64 = 12.0 / PI;

///
/// Radians to minutes
///
pub const R2M: f64 = (12.0 * 60.0) / PI;

///
/// Radians to seconds
///
pub const R2S: f64 = (12.0 * 3600.0) / PI;