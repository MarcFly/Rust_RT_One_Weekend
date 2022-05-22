use crate::objects::prelude::*;
use crate::rtow_math::prelude::*;
use crate::materials::prelude::*;
use std::sync::Arc;
pub enum Objects {
    Sphere(sphere),
    MovingSphere(moving_sphere),
    AABB(aabb),
    Cube(aa_box),
    Moved(translated),
    Rotated(rotated),
    ConstantMedium(constant_medium),
    BVH_Node(bvh_node),
    XY_Rect(xy_rect),
    XZ_Rect(xz_rect),
    YZ_Rect(yz_rect),
    HitList(hittable_list),
}

pub fn object_match_hit(object: &Objects, r: &ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    match object {
        Objects::Sphere(obj) =>                     obj.hit(r, t_min, t_max, rec),
        Objects::MovingSphere(obj) =>        obj.hit(r, t_min, t_max, rec),
        Objects::AABB(obj) =>                         obj.hit(r, t_min, t_max, rec),
        Objects::Cube(obj) =>                       obj.hit(r, t_min, t_max, rec),
        Objects::Moved(obj) =>                  obj.hit(r, t_min, t_max, rec),
        Objects::Rotated(obj) =>                   obj.hit(r, t_min, t_max, rec),
        Objects::ConstantMedium(obj) =>    obj.hit(r, t_min, t_max, rec),
        Objects::BVH_Node(obj) =>                 obj.hit(r, t_min, t_max, rec),
        Objects::XY_Rect(obj) =>                 obj.hit(r, t_min, t_max, rec),
        Objects::XZ_Rect(obj) =>                 obj.hit(r, t_min, t_max, rec),
        Objects::YZ_Rect(obj) =>                 obj.hit(r, t_min, t_max, rec),
        Objects::HitList(obj) =>                 obj.hit(t_min, t_max, rec, r),
        _ => false
    }

}

pub fn object_match_hit_arc(object: &Arc<Objects>, r: &ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    let v = &**object;
    object_match_hit(v, r, t_min, t_max, rec)

}

pub fn object_match_aabb(object: &Objects, time0: f64, time1: f64) -> (bool, aabb) {
    match object {
        Objects::Sphere(obj) =>                     obj.get_aabb(time0, time1),
        Objects::MovingSphere(obj) =>        obj.get_aabb(time0, time1),
        Objects::AABB(obj) =>                         obj.get_aabb(time0, time1),
        Objects::Cube(obj) =>                       obj.get_aabb(time0, time1),
        Objects::Moved(obj) =>                  obj.get_aabb(time0, time1),
        Objects::Rotated(obj) =>                   obj.get_aabb(time0, time1),
        Objects::ConstantMedium(obj) =>    obj.get_aabb(time0, time1),
        Objects::BVH_Node(obj) =>                 obj.get_aabb(time0, time1),
        Objects::XY_Rect(obj) =>                 obj.get_aabb(time0, time1),
        Objects::XZ_Rect(obj) =>                 obj.get_aabb(time0, time1),
        Objects::YZ_Rect(obj) =>                 obj.get_aabb(time0, time1),
        Objects::HitList(obj) =>                 obj.get_aabb(time0, time1),
        _ => (false, aabb::new()),
    }
}

pub fn object_match_aabb_arc(object: &Arc<Objects>, time0: f64, time1: f64) -> (bool, aabb) {
    let v = &**object;
    object_match_aabb(v, time0, time1)
}

pub fn object_match_compare(object: &Objects, obj2: &Arc<Objects>, axis: usize) -> bool {
    match object {
        Objects::Sphere(obj) =>                     obj.compare(obj2, axis),
        Objects::MovingSphere(obj) =>        obj.compare(obj2, axis),
        Objects::AABB(obj) =>                         obj.compare(obj2, axis),
        Objects::Cube(obj) =>                       obj.compare(obj2, axis),
        Objects::Moved(obj) =>                  obj.compare(obj2, axis),
        Objects::Rotated(obj) =>                   obj.compare(obj2, axis),
        Objects::ConstantMedium(obj) =>    obj.compare(obj2, axis),
        Objects::BVH_Node(obj) =>                 obj.compare(obj2, axis),
        Objects::XY_Rect(obj) =>                 obj.compare(obj2, axis),
        Objects::XZ_Rect(obj) =>                 obj.compare(obj2, axis),
        Objects::YZ_Rect(obj) =>                 obj.compare(obj2, axis),
        Objects::HitList(obj) =>                 obj.compare(obj2, axis),
        _ => false,
    }
}
