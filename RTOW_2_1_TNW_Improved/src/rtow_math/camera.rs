use crate::rtow_math::{
    vec3::*, 
    ray::*,
    defines::*,
};

use crate::objects::prelude::*;

#[derive(Copy, Clone)]
pub struct camera {
    origin: point3,
    pitch: vec3, //horizontal
    yaw: vec3, //vertical
    lower_left: point3,
    u: vec3,
    v: vec3,
    w: vec3,
    lens_rad: f64,
    _time0: f64,
    _time1: f64,
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
            u: point3::new(),
            v: point3::new(),
            w: point3::new(),
            lens_rad: 0.,
            _time0: 0.,
            _time1: 0.,
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
            u: point3::new(),
            v: point3::new(),
            w: point3::new(),
            lens_rad: 0.,
            _time0: 0.,
            _time1: 0.,
        }
    }

    pub fn from_fov(aspect_ratio: f64, yfov: f64) -> camera {
        let fov_rads = deg_to_rad(yfov);
        let vp_h = 2. * (fov_rads/2.).tan();
        let vp_w = vp_h * aspect_ratio;

        let origin =  point3::new();
        let pitch = vec3::from(vp_w, 0.,0.);
        let yaw = vec3::from(0., vp_h, 0.);
        let lower_left = origin - pitch /2. - yaw / 2. - vec3::from(0.,0.,1.);

        camera {
            origin,
            pitch,
            yaw,
            lower_left,
            u: point3::new(),
            v: point3::new(),
            w: point3::new(),
            lens_rad: 0.,
            _time0: 0.,
            _time1: 0.,
        }
    }

    pub fn from_look(from: point3, lookat: point3, vup: vec3, yfov: f64, aspect_ratio: f64) -> camera {
        let fov_rads = deg_to_rad(yfov);
        let vp_h = 2. * (fov_rads/2.).tan();
        let vp_w = vp_h * aspect_ratio;

        let w = (from - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u).unit_vec();

        let origin = from;
        let pitch = u * vp_w;
        let yaw = v * vp_h;
        let lower_left = origin - pitch / 2. - yaw / 2. - w;

        camera {
            origin,
            pitch,
            yaw,
            lower_left,
            u: point3::new(),
            v: point3::new(),
            w: point3::new(),
            lens_rad: 0.,
            _time0: 0.,
            _time1: 0.,
        }
    }

    pub fn from_all(from: point3, lookat: point3, vup: vec3, yfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, _time0: f64, _time1: f64) -> camera {
        let fov_rads = deg_to_rad(yfov);
        let vp_h = 2. * (fov_rads/2.).tan();
        let vp_w = vp_h * aspect_ratio;

        let w = (from - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u).unit_vec();

        let origin = from;
        let pitch = u * vp_w * focus_dist;
        let yaw = v * vp_h * focus_dist;
        let lower_left = origin - pitch / 2. - yaw / 2. - w * focus_dist;
        let lens_rad = aperture / 2.;

        camera {
            origin,
            pitch,
            yaw,
            lower_left,
            u,
            v,
            w,
            lens_rad,
            _time0,
            _time1,
        }
    }

    pub fn ray(&self, u: f64, v:f64) -> ray {
        let dir = self.lower_left + self.pitch*u + self.yaw*v - self.origin;
        ray::from(self.origin, dir)
    }
    
    pub fn time_ray(&self, u: f64, v: f64) -> ray {
        let dir = self.lower_left + self.pitch*u + self.yaw*v - self.origin;
        ray::from_t(self.origin, dir, rand_f64_r(self._time0, self._time1))
    }

    pub fn focus_ray(&self, u: f64, v:f64) -> ray {
        let rd = random_in_cylindermap() * self.lens_rad;
        let offset = self.u * *rd.x() + self.v * *rd.y();
        let og = self.origin + offset;
        let dir = self.lower_left + self.pitch*u + self.yaw*v - self.origin - offset;
        ray::from(og, dir)
    }

    pub fn focus_time_ray(&self, u: f64, v:f64) -> ray {
        let rd = random_in_cylindermap() * self.lens_rad;
        let offset = self.u * *rd.x() + self.v * *rd.y();
        let og = self.origin + offset;
        let dir = self.lower_left + self.pitch*u + self.yaw*v - self.origin - offset;
        ray::from_t(self.origin, dir, rand_f64_r(self._time0, self._time1))
    }
}