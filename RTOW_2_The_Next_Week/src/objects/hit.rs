use crate::rtow_math::ray::*;
use crate::rtow_math::vec3::*;
use crate::objects::sphere::*;
use crate::materials::*;
use crate::objects::aabb::*;
use crate::rtow_math::vec2::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct hit_record {
    pub p: point3,
    pub n: vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    pub iters: i32,
    pub uv: point2,
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
                iters: 0,
                uv: point2::new(),
            }
        }
    
    pub fn set_face_normal(&mut self, r: &ray) {
        self.front_face = r.dir.dot(&self.n) < 0.;
        self.n = if self.front_face {self.n} else {self.n * -1.};
    }
}

use crate::objects::hittable_list::*;
use std::sync::Mutex;
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool;
    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb);

    fn compare(&self, other: Arc<dyn Hittable>, axis: usize) -> bool {
        let (check, box1) = self.get_aabb(0., 0.);
        let (check2, box2) = other.get_aabb(0., 0.);

        if !check || !check2 { panic!("Passed value without aabb!!!!")};

        box1.min.v[axis] < box2.min.v[axis]
    }

    //fn get_uv(&self, hit_pos: &point3, uv: &mut point2);
}

pub fn compare_x(main: &Arc<dyn Hittable>, other: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    let (check, box1) = main.get_aabb(0., 0.);
    let (check2, box2) = other.get_aabb(0., 0.);

    if !check || !check2 { panic!("Passed value without aabb!!!!")};
    
    if box1.min.v[0] < box2.min.v[0] { std::cmp::Ordering::Less }
    else { std::cmp::Ordering::Greater }
}

pub fn compare_y(main: &Arc<dyn Hittable>, other: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    let (check, box1) = main.get_aabb(0., 0.);
    let (check2, box2) = other.get_aabb(0., 0.);

    if !check || !check2 { panic!("Passed value without aabb!!!!")};

    if box1.min.v[1] < box2.min.v[1] { std::cmp::Ordering::Less }
    else { std::cmp::Ordering::Greater }
}

pub fn compare_z(main: &Arc<dyn Hittable>, other: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    let (check, box1) = main.get_aabb(0., 0.);
    let (check2, box2) = other.get_aabb(0., 0.);

    if !check || !check2 { panic!("Passed value without aabb!!!!")};

    if box1.min.v[2] < box2.min.v[2] { std::cmp::Ordering::Less }
    else { std::cmp::Ordering::Greater }
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