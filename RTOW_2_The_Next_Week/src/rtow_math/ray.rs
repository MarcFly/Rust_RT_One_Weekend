use crate::rtow_math::vec3::*;

pub struct ray {
    pub origin: point3,
    pub dir: vec3,
    pub time: f64, // Add Time for motion blur -> SpatioTemporal ray tracing
}

impl ray {
    pub fn new() -> ray { ray{origin: point3::new(), dir: vec3::new(), time: 0.}}
    pub fn from(p: point3, d: vec3) -> ray { ray {origin: p, dir: d, time: 0. }}
    pub fn from_t(p: point3, d: vec3, time: f64) -> ray { ray { origin: p, dir: d, time } }
    pub fn at(&self, t: f64) -> point3 { 
        self.origin + self.dir * t 
    }
}