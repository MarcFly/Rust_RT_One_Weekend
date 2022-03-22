use crate::rtow_math::ray::*;
use crate::rtow_math::vec3::*;
use crate::rtow_math::sphere::*;
use crate::materials::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct hit_record {
    pub p: point3,
    pub n: vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl hit_record {
    pub fn new() -> hit_record { 
        let new_def = Default{};
            hit_record { 
                p: point3::new(), 
                n: vec3::new(), 
                t: std::f64::INFINITY, 
                front_face: true,
                mat: Arc::new(new_def),
            }
        }
    
    pub fn set_face_normal(&mut self, r: &ray) {
        self.front_face = r.dir.dot(&self.n) < 0.;
        self.n = if self.front_face {self.n} else {self.n * -1.};
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool;
}



pub fn hit_list(hittables: &Vec<Box<dyn Hittable>>, t_min: f64, t_max: f64, rec: &mut hit_record, r: &ray) -> bool {
    let mut temp_rec = hit_record::new();
    let mut hit_anything = false;
    let mut closest = t_max;

    for obj in hittables {
        if(obj.hit(r, t_min, t_max, &mut temp_rec) && temp_rec.t < closest) {
            hit_anything = true;
            closest = temp_rec.t;
            *rec = temp_rec.clone();
        }
    }
    hit_anything
}