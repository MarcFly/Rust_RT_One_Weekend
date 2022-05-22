use crate::objects::hit::*;
use std::sync::Arc;
use std::sync::Mutex;
use crate::rtow_math::ray::*;
use crate::objects::aabb::*;
use crate::rtow_math::rng::*;

use crate::materials::*;

enum Node {
    Branch(Arc<dyn Hittable>),
    Tree(Box<bvh_node>),
}




pub struct hittable_list {
    pub obj_list: Box<Vec<Arc<dyn Hittable>>>,
    bvh_start: Node,
    //bvh_node_list: Box<Vec<Arc<dyn Hittable>>>,
    pub num_nodes: i32,
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            obj_list: Box::new(Vec::new()),
            bvh_start: Node::Tree(Box::new(bvh_node::new_empty())),
            //bvh_node_list: Box::new(Vec::new()),
            num_nodes: 0,
        }
    }

    pub fn hit_d(&self, t_min: f64, t_max: f64, rec: &mut hit_record, r: &ray) -> bool {
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
        
        let hit_confirm = match &self.bvh_start {
            Node::Branch(item) => item.hit(r, t_min, t_max, &mut temp_rec),
            Node::Tree(item) => item.hit(r, t_min, t_max, &mut temp_rec),
        };
        if( temp_rec.t < closest && hit_confirm ) {
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
        let node = Box::new(bvh_node::new(&mut self.obj_list[..], time0, time1, &mut self.num_nodes,  0));
        //self.bvh_node_list.push(arc_node);
        //let len = self.bvh_node_list.len();
        self.bvh_start = Node::Tree( node);
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

struct bvh_node {
    left_ch: Node,
    right_ch: Node,
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
            left_ch: Node::Branch(Arc::new(aabb::new())), 
            right_ch: Node::Branch(Arc::new(aabb::new())), 
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
    pub fn new(obj_list: &mut [Arc<dyn Hittable>], time0: f64, time1: f64, num_nodes: &mut i32, depth: i32 ) -> bvh_node {
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

        ret_node.internal_depth = depth;

        //println!("{:?}", obj_list);
        if list_len == 1 {
            ret_node.left_ch = Node::Branch(Arc::clone(&obj_list[0]));
            ret_node.right_ch = Node::Branch(Arc::clone(&obj_list[0]));
        } else if list_len == 2 {
            if obj_list[0].compare(Arc::clone(&obj_list[1]), axis) {
                ret_node.left_ch = Node::Branch(Arc::clone(&obj_list[0]));
                ret_node.right_ch = Node::Branch(Arc::clone(&obj_list[1]));
            } else {
                ret_node.left_ch = Node::Branch(Arc::clone(&obj_list[1]));
                ret_node.right_ch = Node::Branch(Arc::clone(&obj_list[0]));
            }
        } else {
            obj_list.sort_by(|a, b| compare_fun(a,b));
            let mid = list_len / 2;
            let node_l = bvh_node::new(&mut obj_list[0..mid], time0, time1, num_nodes, depth + 1 );
            
            ret_node.left_ch = Node::Tree(Box::new(node_l));

            let node_r = bvh_node::new(&mut obj_list[mid..], time0, time1, num_nodes, depth + 1);
            ret_node.right_ch = Node::Tree(Box::new(node_r));
        }

        let (check1, box_l) = match &ret_node.left_ch {
            Node::Branch(item) => item.get_aabb(time0, time1),
            Node::Tree(item) => item.get_aabb(time0, time1),
        };

        let (check2, box_r) = match &ret_node.right_ch {
            Node::Branch(item) => item.get_aabb(time0, time1),
            Node::Tree(item) => item.get_aabb(time0, time1),
        };
        if !check1 || !check2 { panic!("BVH_Node had an invalid aabb, light?")};
        ret_node.aabb_box = aabb::from_2_aabb(box_l, box_r);
        *num_nodes += 1;

        ret_node
    }
}

use tracing::{debug, event, info, info_span, span, Level};

unsafe impl Send for bvh_node {}
unsafe impl Sync for bvh_node {}
impl Hittable for bvh_node {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        let span_bvh_node_hit = span!(Level::TRACE, "BVH_NODE_HIT"); //, self.internal_depth);
        let span_bvh_node_hit = span_bvh_node_hit.enter();

        if !self.aabb_box.hit(r, t_min, t_max, rec) { return false };

        event!(Level::TRACE, "BVH_NODE_LEFT");
        let hit_left = match &self.left_ch {
            Node::Tree(item) => item.hit(r, t_min, t_max, rec),
            Node::Branch(item) => item.hit(r,t_min, t_max, rec),
        };
        //if hit_left {return true};
        
        event!(Level::TRACE, "BVH_NODE_RIGHT");
        
        let hit_right = match &self.right_ch {
            Node::Tree(item) => item.hit(r, t_min, if hit_left {rec.t} else {t_max}, rec),
            Node::Branch(item) => item.hit(r,t_min, if hit_left {rec.t} else {t_max}, rec),
        };

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