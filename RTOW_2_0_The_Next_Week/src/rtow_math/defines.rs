pub use crate::rtow_math::rng::*;
pub use crate::rtow_math::vec3::*;

pub use std::f64::INFINITY;

pub const pi: f64 = 3.1415926535897932385;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * pi / 180.
}

pub fn rad_to_def(radians: f64) -> f64 {
    radians * 180. / pi
}

pub fn random_in_disk() -> point3 {
    while(true) {
        let p = point3::from(
            rand_f64_r(-1., 1.),
            rand_f64_r(-1., 1.),
            0. );
        if p.length_squared()  < 1. { return p }
    };
    vec3::new() // Something went terribly wrong;
}

#[macro_use]
use lazy_static::*;

// This is a cube inscripted in a sphere of radius 1
lazy_static! {
    pub static ref side_max: f64 = { 1. / (3.0 as f64).sqrt() / 2.0};
    pub static ref side_min: f64 = { -1. / (3.0 as f64).sqrt() / 2.0};
    pub static ref side_sum: f64 = {(1. - ( 1. / (3.0 as f64).sqrt())) / 2.};
}