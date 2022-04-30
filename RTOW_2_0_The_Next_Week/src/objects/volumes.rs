use crate::rtow_math::prelude::*;
use crate::objects::prelude::*;
use crate::materials::prelude::*;
use std::sync::Arc;

pub struct constant_medium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl constant_medium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> constant_medium {
        let neg_inv_density = (-1. / density);
        let phase_function = Arc::new(isotropic::new(tex));

        constant_medium { boundary, phase_function, neg_inv_density }
    }
}

unsafe impl Sync for constant_medium {}
unsafe impl Send for constant_medium {}

impl Hittable for constant_medium {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        let (mut rec1, mut rec2) = (hit_record::new(), hit_record::new());

        // Default check, if t_min were to be inside volume, we don't hit 
        // But volumes have constant hit so we must check along all the ray
        if(!self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1)) {
            return false
        }

        // Here we actually check at which point we are hitting
        // aka, how much of the volume we have traversed or not
        if(!self.boundary.hit(r, rec1.t+0.0001, INFINITY, &mut rec2)) {
            return false
        }

        // However, we are assuming that from the inside, we only escape once
        // Does not check for multiple hits like in a torus or other shapes that have voids
        // or holes in them!

        if rec1.t < t_min {rec1.t = t_min};
        if rec2.t > t_max {rec2.t = t_max};

        if rec1.t < 0. {rec1.t = 0.};

        let r_len  = r.dir.length();
        let dist_in_boundary = (rec2.t - rec1.t) * r_len;
        let hit_dist = self.neg_inv_density * rand_f64().ln();

        if hit_dist > dist_in_boundary {return false};

        rec.t = rec1.t + hit_dist / r_len;
        rec.p = r.at(rec.t);

        rec.n = vec3::up();
        rec.front_face = true;
        rec.mat = Arc::clone(&self.phase_function);

        true
    }


    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        self.boundary.get_aabb(time0, time1)
    }
}