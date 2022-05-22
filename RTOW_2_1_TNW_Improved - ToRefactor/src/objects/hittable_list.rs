use crate::objects::hit::*;
use std::sync::Arc;
use std::sync::Mutex;
use crate::rtow_math::ray::*;
use crate::objects::aabb::*;
use crate::rtow_math::rng::*;
use crate::objects::prelude::*;
pub struct hittable_list {
    pub obj_list: Box<Vec<Arc<Objects>>>,
    bvh_start: Arc<Objects>,
    bvh_node_list: Box<Vec<Arc<Objects>>>,
    pub num_nodes: i32,
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            obj_list: Box::new(Vec::new()),
            bvh_start: Arc::new(Objects::BVH_Node(bvh_node::new_empty())),
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
            if(object_match_hit(&obj, r, t_min, t_max, &mut temp_rec) && temp_rec.t < closest) {
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
    
        if let Objects::BVH_Node(node) = &*self.bvh_start {
                if (node.hit(r, t_min, t_max, &mut temp_rec) && temp_rec.t < closest) {
                hit_anything = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
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
            (check, temp_aabb) = object_match_aabb(&obj, time0, time1);
            if !check { continue };
            ret_aabb = if first_b {temp_aabb} else {aabb::from_2_aabb(ret_aabb, temp_aabb)};
            first_b = false;
        }
        (true, ret_aabb)
    }

    pub fn construct_bvh(&mut self, time0: f64, time1: f64) {
        let arc_node = bvh_node::new(&mut self.obj_list[..], time0, time1, &mut self.num_nodes, &mut self.bvh_node_list, 0);
        self.bvh_node_list.push(Arc::new(Objects::BVH_Node(arc_node)));
        let len = self.bvh_node_list.len();
        self.bvh_start = Arc::clone(&self.bvh_node_list[len-1]);
    }
}

impl Hittable for hittable_list {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        self.hit_bvh(t_min, t_max, rec, r)
    }
    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        self.compute_full_aabb(time0, time1)
    }
}

use crate::materials::*;
pub struct bvh_node {
    left_ch: Arc<Objects>,
    right_ch: Arc<Objects>,
    pub aabb_box: aabb,
    pub mat: Arc<dyn Material>,
    pub internal_depth: i32,
}

use crate::objects::hit::*;
use crate::rtow_math::vec3::*;
use crate::materials::textures::*;

impl bvh_node {
    pub fn new_empty() -> bvh_node {
        bvh_node { 
            left_ch: Arc::new(Objects::AABB(aabb::new())), 
            right_ch: Arc::new(Objects::AABB(aabb::new())), 
            aabb_box: aabb::new(),
            mat: Arc::new(dielectric{
                albedo: colorRGB::from(1.,0.,0.), 
                alpha: 0.1, 
                index_refr: 0.,
                tex: Arc::new(Solid_Color::new()),
            }),
            internal_depth: 0,
        }
    }
    pub fn new(obj_list: &mut [Arc<Objects>], time0: f64, time1: f64, num_nodes: &mut i32, node_list: &mut Box<Vec<Arc<Objects>>>, depth: i32 ) -> bvh_node {
        //let axis = rand_i8_r(0,2) as usize;
        let axis = 1;
        let mut ret_node = bvh_node::new_empty();
        let list_len = obj_list.len();
        let compare_fun = match axis {
            0 => compare_x,
            1 => compare_y,
            2 => compare_z,
            _ => panic!("How did oyu get an axis < 0 or > 2 with a rng with range 0..2 ???")
        };

        ret_node.internal_depth = depth;

        //println!("{:?}", obj_list);
        if list_len == 1 {
            ret_node.left_ch = Arc::clone(&obj_list[0]);
            ret_node.right_ch = Arc::clone(&obj_list[0]);
        } else if list_len == 2 {
            if object_match_compare(&obj_list[0], &Arc::clone(&obj_list[1]), axis) {
                ret_node.left_ch = Arc::clone(&obj_list[0]);
                ret_node.right_ch = Arc::clone(&obj_list[1]);
            } else {
                ret_node.left_ch = Arc::clone(&obj_list[1]);
                ret_node.right_ch = Arc::clone(&obj_list[0]);
            }
        } else {
            obj_list.sort_by(|a, b| compare_fun(a,b));
            let mid = list_len / 2;
            let arc_node_l = bvh_node::new(&mut obj_list[0..mid], time0, time1, num_nodes, node_list, depth + 1 );
            node_list.push(Arc::new(Objects::BVH_Node(arc_node_l)));
            ret_node.left_ch = Arc::clone(&node_list[node_list.len()-1]);
            
            let arc_node_r = bvh_node::new(&mut obj_list[mid..], time0, time1, num_nodes, node_list, depth + 1);
            node_list.push(Arc::new(Objects::BVH_Node(arc_node_r)));
            ret_node.right_ch = Arc::clone(&node_list[node_list.len()-1]);
        }

        let (check1, box_l) = object_match_aabb_arc(&ret_node.left_ch, time0, time1);
        let (check2, box_r) = object_match_aabb_arc(&ret_node.right_ch, time0, time1);
        if !check1 || !check2 { panic!("BVH_Node had an invalid aabb, light?")};
        ret_node.aabb_box = aabb::from_2_aabb(box_l, box_r);
        *num_nodes += 1;

        ret_node
    }
}

//use tracing::{debug, event, info, info_span, span, Level};

unsafe impl Send for bvh_node {}
unsafe impl Sync for bvh_node {}
impl Hittable for bvh_node {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        //let span_bvh_node_hit = span!(Level::TRACE, "BVH_NODE_HIT");
        //let span_bvh_node_hit = span_bvh_node_hit.enter();

        if !self.aabb_box.hit(r, t_min, t_max, rec) { return false };

        //event!(Level::TRACE, "BVH_NODE_LEFT");
        let hit_left = object_match_hit_arc(&self.left_ch, r, t_min, t_max, rec);
        if hit_left {return true};
        
        //event!(Level::TRACE, "BVH_NODE_RIGHT");

        object_match_hit_arc(&self.right_ch, r, t_min, if hit_left {rec.t} else {t_max}, rec)        
        
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