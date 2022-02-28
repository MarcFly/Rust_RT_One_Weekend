use crate::rtow_math::vec3::*;

pub struct ray {
    pub origin: point3,
    pub dir: vec3,
}

impl ray {
    pub fn new() -> ray { ray{origin: point3::new(), dir: vec3::new()}}
    pub fn from(p: &point3, d: &vec3) -> ray{ ray{origin: point3::from_vec(p), dir: vec3::from_vec(d)}}

    pub fn at(mut self, t: f64) -> point3 { 
        let movement = self.dir.mult_sc(t);
        self.origin.add(&movement) 
    }
}