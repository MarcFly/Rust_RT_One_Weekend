
pub use std::f64::INFINITY;

pub const pi: f64 = 3.1415926535897932385;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * pi / 180.
}

pub fn rad_to_def(radians: f64) -> f64 {
    radians * 180. / pi
}