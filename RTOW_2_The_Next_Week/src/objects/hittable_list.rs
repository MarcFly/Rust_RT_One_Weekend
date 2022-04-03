use crate::objects::hit::*;
use std::sync::Arc;
use std::sync::Mutex;
use crate::rtow_math::ray::*;
use crate::objects::aabb::*;
use crate::rtow_math::rng::*;

pub struct hittable_list {
    pub obj_list: Box<Vec<Arc<dyn Hittable>>>,
    bvh_start: Arc<dyn Hittable>,
    bvh_node_list: Box<Vec<Arc<dyn Hittable>>>,
    pub num_nodes: i32,
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            obj_list: Box::new(Vec::new()),
            bvh_start: Arc::new(bvh_node::new_empty()),
            bvh_node_list: Box::new(Vec::new()),
            num_nodes: 0,
        }
    }

    pub fn hit(&self, t_min: f64, t_max: f64, rec: &mut hit_record, r: &ray) -> bool {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest = t_max;
    
        for obj in self.obj_list.as_ref() {
            rec.iters += 1;
            if(obj.hit(r, t_min, t_max, &mut temp_rec) && temp_rec.t < closest) {
                hit_anything = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    pub fn hit_bvh(&self, t_min: f64, t_max: f64, rec: &mut hit_record, r: &ray) -> bool {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest = t_max;
    
        if(self.bvh_start.hit(r, t_min, t_max, &mut temp_rec) && temp_rec.t < closest) {
            hit_anything = true;
            closest = temp_rec.t;
            *rec = temp_rec.clone();
        }
        
        hit_anything
    }

    pub fn compute_full_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        if self.obj_list.len() == 0 { return (false, aabb::new()) }
        let mut ret_aabb = aabb::new();
        let mut temp_aabb = aabb::new();
        let mut first_b = true;
        let mut check: bool;
        for obj in self.obj_list.as_ref() {
            (check, temp_aabb) = obj.get_aabb(time0, time1);
            if !check { continue };
            ret_aabb = if first_b {temp_aabb} else {aabb::from_2_aabb(ret_aabb, temp_aabb)};
            first_b = false;
        }
        (true, ret_aabb)
    }

    pub fn construct_bvh(&mut self, time0: f64, time1: f64) {
        let arc_node = Arc::new(bvh_node::new(&mut self.obj_list[..], time0, time1, &mut self.num_nodes, &mut self.bvh_node_list));
        self.bvh_node_list.push(arc_node);
        let len = self.bvh_node_list.len();
        self.bvh_start = Arc::clone(&self.bvh_node_list[len-1]);
    }
}

use crate::materials::*;
struct bvh_node {
    left_ch: Arc<dyn Hittable>,
    right_ch: Arc<dyn Hittable>,
    pub aabb_box: aabb,
    pub mat: Arc<dyn Material>,
}

use crate::objects::hit::*;
use crate::rtow_math::vec3::*;

impl bvh_node {
    pub fn new_empty() -> bvh_node {
        bvh_node { 
            left_ch: Arc::new(aabb::new()), 
            right_ch: Arc::new(aabb::new()), 
            aabb_box: aabb::new(),
            mat: Arc::new(dielectric{
                albedo: colorRGB::from(1.,0.,0.), 
                alpha: 0.1, 
                index_refr: 0.
            })
        }
    }
    pub fn new(obj_list: &mut [Arc<dyn Hittable>], time0: f64, time1: f64, num_nodes: &mut i32, node_list: &mut Box<Vec<Arc<dyn Hittable>>> ) -> bvh_node {
        //let axis = rand_i8_r(0,2) as usize;
        let axis = 0;
        let mut ret_node = bvh_node::new_empty();
        let list_len = obj_list.len();
        let compare_fun = match axis {
            0 => compare_x,
            1 => compare_y,
            2 => compare_z,
            _ => panic!("How did oyu get an axis < 0 or > 2 with a rng with range 0..2 ???")
        };

        //println!("{:?}", obj_list);
        if list_len == 1 {
            ret_node.left_ch = Arc::clone(&obj_list[0]);
            ret_node.right_ch = Arc::clone(&obj_list[0]);
        } else if list_len == 2 {
            if obj_list[0].compare(Arc::clone(&obj_list[1]), axis) {
                ret_node.left_ch = Arc::clone(&obj_list[0]);
                ret_node.right_ch = Arc::clone(&obj_list[1]);
            } else {
                ret_node.left_ch = Arc::clone(&obj_list[1]);
                ret_node.right_ch = Arc::clone(&obj_list[0]);
            }
        } else {
            obj_list.sort_by(|a, b| compare_fun(a,b));
            let mid = list_len / 2;
            let arc_node_l = Arc::new(bvh_node::new(&mut obj_list[0..mid], time0, time1, num_nodes, node_list));
            node_list.push(arc_node_l);
            ret_node.left_ch = Arc::clone(&node_list[node_list.len()-1]);
            
            let arc_node_r = Arc::new(bvh_node::new(&mut obj_list[mid..], time0, time1, num_nodes, node_list));
            node_list.push(arc_node_r);
            ret_node.right_ch = Arc::clone(&node_list[node_list.len()-1]);
        }

        let (check1, box_l) = ret_node.left_ch.get_aabb(time0, time1);
        let (check2, box_r) = ret_node.right_ch.get_aabb(time0, time1);
        if !check1 || !check2 { panic!("BVH_Node had an invalid aabb, light?")};
        ret_node.aabb_box = aabb::from_2_aabb(box_l, box_r);
        *num_nodes += 1;
        ret_node
    }
}

unsafe impl Send for bvh_node {}
unsafe impl Sync for bvh_node {}
impl Hittable for bvh_node {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        if !self.aabb_box.hit(r, t_min, t_max, rec) { return false };

        let hit_left = self.left_ch.hit(r, t_min, t_max, rec);
        if(hit_left){
            let t_debug = true;
        }
        let hit_right = self.right_ch.hit(r, t_min, if hit_left {rec.t} else {t_max}, rec);
        
        hit_left || hit_right
    }

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (true, aabb::from(self.aabb_box.min, self.aabb_box.max))
    }
}

//impl Drop for bvh_node {
//    fn drop(&mut self) {
//
//    }
//}