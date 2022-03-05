use crate::rtow_math::{
    vec3::*, 
    ray::*,
};

pub struct camera {
    origin: point3,
    pitch: vec3, //horizontal
    yaw: vec3, //vertical
    lower_left: point3,
}

impl camera {
    pub fn new() -> camera {
        let aspect_ratio = 16.0 / 9.0;
        let vp_h = 2.0;
        let vp_w = aspect_ratio * vp_h;
        let focal_length = 1.0;

        let origin =  point3::new();
        let pitch = vec3::from(vp_w, 0.,0.);
        let yaw = vec3::from(0., vp_h, 0.);
        let lower_left = origin - pitch /2. - yaw / 2. + vec3::from(0.,0.,focal_length);

        camera {
            origin,
            pitch, // horizontal
            yaw, // vertical
            lower_left,
        }
    }

    pub fn from(aspect_ratio: f64, vp_w: f64, focal_length: f64) -> camera {
        let vp_h = vp_w / aspect_ratio;


        let origin =  point3::new();
        let pitch = vec3::from(vp_w, 0.,0.);
        let yaw = vec3::from(0., vp_h, 0.);
        let lower_left = origin - pitch /2. - yaw / 2. - vec3::from(0.,0.,focal_length);

        camera {
            origin,
            pitch,
            yaw,
            lower_left,
        }
    }

    pub fn ray(&self, u: f64, v:f64) -> ray {
        let dir = self.lower_left + self.pitch*u + self.yaw*v - self.origin;
        ray::from(&self.origin, &dir)
    }
}