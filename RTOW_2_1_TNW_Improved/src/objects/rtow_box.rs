use crate::rtow_math::prelude::*;
use crate::objects::prelude::*;
use crate::materials::prelude::*;
use std::sync::Arc;

pub struct aa_box {
    min: point3,
    max: point3,
    sides: hittable_list,
}

impl aa_box {
    pub fn from(p0: point3, p1: point3, mat: Arc<dyn Material>) -> aa_box {
        let mut ret = aa_box{
            min: p0, 
            max: p1, 
            sides: hittable_list::new()
        };

        ret.sides.obj_list.push(Arc::new(xy_rect::from(p0.v[0], p1.v[0], p0.v[1], p1.v[1], p1.v[2], Arc::clone(&mat))));
        ret.sides.obj_list.push(Arc::new(xy_rect::from(p0.v[0], p1.v[0], p0.v[1], p1.v[1], p0.v[2], Arc::clone(&mat))));
        ret.sides.obj_list.push(Arc::new(xz_rect::from(p0.v[0], p1.v[0], p0.v[2], p1.v[2], p1.v[1], Arc::clone(&mat))));
        ret.sides.obj_list.push(Arc::new(xz_rect::from(p0.v[0], p1.v[0], p0.v[2], p1.v[2], p0.v[1], Arc::clone(&mat))));
        ret.sides.obj_list.push(Arc::new(yz_rect::from(p0.v[1], p1.v[1], p0.v[2], p1.v[2], p1.v[0], Arc::clone(&mat))));
        ret.sides.obj_list.push(Arc::new(yz_rect::from(p0.v[1], p1.v[1], p0.v[2], p1.v[2], p0.v[0], Arc::clone(&mat))));

        ret.sides.construct_bvh(0.,1.);

        ret
    }
}

impl Hittable for aa_box {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        self.sides.hit_bvh(t_min, t_max, rec, r)
    }

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (
            true,
            aabb::from(self.min, self.max)
        )
    }
}