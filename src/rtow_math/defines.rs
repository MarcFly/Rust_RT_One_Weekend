
pub use std::f64::INFINITY;

pub const pi: f64 = 3.1415926535897932385;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * pi / 180.
}

pub fn rad_to_def(radians: f64) -> f64 {
    radians * 180. / pi
}

#[macro_use]
use lazy_static::*;

lazy_static! {
    pub static ref side_max: f64 = { 1. / (3.0 as f64).sqrt() / 2.0};
    pub static ref side_min: f64 = { -1. / (3.0 as f64).sqrt() / 2.0};
    pub static ref side_sum: f64 = {(1. - ( 1. / (3.0 as f64).sqrt())) / 2.};
}