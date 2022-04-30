use std::f32::INFINITY;

use crate::rtow_math::prelude::*;
use crate::objects::prelude::*;

use std::sync::Arc;

pub struct translated {
    obj: Box<dyn Hittable>,
    offset: vec3,
}

impl translated {
    pub fn new( obj: Box<dyn Hittable>, offset: vec3) -> translated { translated{obj,offset} }
}

impl Hittable for translated {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        let mov_r = ray::from_t(r.origin - self.offset, r.dir, r.time);
        if !self.obj.hit(&mov_r, t_min, t_max, rec) {
            return false
        };

        rec.p = rec.p + self.offset;
        rec.set_face_normal(&mov_r, rec.n);
        true

    }
    
    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        let (check, ret_aabb) = self.obj.get_aabb(time0, time1);
        (
            check,
            aabb::from(ret_aabb.min + self.offset, ret_aabb.max + self.offset)
        )
    }
}

pub struct rotated {
    obj: Box<dyn Hittable>,
    angles: point3,
    theta_vals: vec2,
    hasbox: bool,
    bbox: aabb,
}

impl rotated {
    pub fn new(obj: Box<dyn Hittable>, angles: vec3) -> rotated {
        let mut ret = rotated { obj, angles, theta_vals: vec2::new(), hasbox: false, bbox: aabb::new() };
        ret.set_rot_y();

        ret
    }   
    pub fn set_rot_y(&mut self) {
        let rads = deg_to_rad(self.angles.v[1]);
        self.theta_vals.v[1] = rads.sin();
        self.theta_vals.v[0] = rads.cos();
        (self.hasbox, self.bbox) = self.obj.get_aabb(0., 1.);
        let mut v_max = point3::inf_min();
        let mut v_min = point3::inf_max();

        // The rotation?
        for i in 0..2 {
            let f64_i = i as f64;
            for j in 0..2 {
                let f64_j = j as f64;
                for k in 0..2 {
                    let f64_k = k as f64;
                    let x = self.bbox.max.v[0] * f64_i + self.bbox.min.v[0] * (1. - f64_i);
                    let y = self.bbox.max.v[1] * f64_j + self.bbox.min.v[1] * (1. - f64_j);
                    let z = self.bbox.max.v[2] * f64_k + self.bbox.min.v[2] * (1. - f64_k);
                    
                    let newx = self.theta_vals.v[0] * x + self.theta_vals.v[1] * z; // cos*x + sin*z
                    let newz = -self.theta_vals.v[1] * x + self.theta_vals.v[0] * z; // -sin*x + cos*z
                    let tester = vec3::from(newx, y, newz);
                    for c in 0..3 {
                        v_min.v[c] = v_min.v[c].min(tester.v[c]);
                        v_max.v[c] = v_max.v[c].max(tester.v[c]);
                    }
                }
            }
        }

        self.bbox = aabb::from(v_min, v_max);
    }
}

impl Hittable for rotated {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        let mut og = r.origin;
        let mut dir = r.dir;

        // Inverse Rotate over Y Axis
        // Inverse here because we are assuming the space is rotate from local to out,
        // So we are reversing that to go into local
        // Full inversion means swapping sin/cosine per x/z values
        // z = cos*z + sin*x
        // x = -sin*z + cos*x   
        og.v[2] = self.theta_vals.v[0] * r.origin.v[2] + self.theta_vals.v[1] * r.origin.v[0]; // cos*x + sin*z
        og.v[0] = -self.theta_vals.v[1] * r.origin.v[2] + self.theta_vals.v[0] * r.origin.v[0]; // -sin*x + cos*z
        
        dir.v[2] = self.theta_vals.v[0] * r.dir.v[2] + self.theta_vals.v[1] * r.dir.v[0]; // cos*x + sin*z
        dir.v[0] = -self.theta_vals.v[1] * r.dir.v[2] + self.theta_vals.v[0] * r.dir.v[0]; // -sin*x + cos*z
        // Other Rotations??
        let rotated_r = ray::from_t(og, dir, r.time);
        if !self.obj.hit(&rotated_r, t_min, t_max, rec) { return false };

        let mut p = rec.p;
        let mut n = rec.n;

        // Rotate over Y Axis
        // This one is not inverted because we are modifying in real world according to the
        // local object, as the n and p will be used in real world for the next bounce
        p.v[0] = self.theta_vals.v[0] * rec.p.v[0] + self.theta_vals.v[1] * rec.p.v[2]; // cos*x + sin*z
        p.v[2] = -self.theta_vals.v[1] * rec.p.v[0] + self.theta_vals.v[0] * rec.p.v[2]; // -sin*x + cos*z
        
        n.v[0] = self.theta_vals.v[0] * rec.n.v[0] + self.theta_vals.v[1] * rec.n.v[2]; // cos*x + sin*z
        n.v[2] = -self.theta_vals.v[1] * rec.n.v[0] + self.theta_vals.v[0] * rec.n.v[2]; // -sin*x + cos*z

        rec.p = p;
        rec.set_face_normal(&rotated_r, n);

        true
        
    }

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (self.hasbox, self.bbox.clone())
    }

}