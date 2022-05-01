use crate::rtow_math::vec3::*;
use crate::rtow_math::vec2::*;
use crate::rtow_math::ray::*;
use crate::objects::hit::*;

pub fn hit_aabb(min: point3, max: point3, ray: &ray) -> f64 {
    let v_x = vec2::from(*min.x(), *max.x()).crossB();
    let v_y = vec2::from(*min.y(), *max.y()).crossB();
    let v_z = vec2::from(*min.z(), *max.z()).crossB();
    
    let r_x = vec2::from(0., *ray.dir.x());
    let r_y = vec2::from(0., *ray.dir.y());
    let r_z = vec2::from(0., *ray.dir.z());
    
    3.
} 

use crate::materials::*;
use crate::materials::textures::*;

#[derive(Clone)]
pub struct aabb {
    pub min: point3,
    pub max: point3,
    mat: Arc<dyn Material>
}

impl aabb {
    pub fn new() -> aabb { 
        aabb {
            min: point3::new(), 
            max: point3::new(),
            mat: Arc::new(dielectric{
                albedo: colorRGB::from(1.,0.,0.), 
                alpha: 0.1, 
                index_refr: 0.,
                tex: Arc::new(Solid_Color::new()),
            })
        }
    }
    pub fn from(min: point3, max: point3) -> aabb { aabb{min,max, mat: Arc::new(dielectric{
        albedo: colorRGB::from(1.,0.,0.), 
        alpha: 0.1, 
        index_refr: 0.,
        tex: Arc::new(Solid_Color::new()),
    })} }

    pub fn from_2_aabb(b1: aabb, b2: aabb) -> aabb {
        let mut ret_b = aabb::new();
        for i in 0..3 {
            ret_b.min.v[i] = b1.min.v[i].min(b2.min.v[i]);
            ret_b.max.v[i] = b1.max.v[i].max(b2.max.v[i]);
        }

        ret_b
        
    }

    pub fn hit_understandable(&self, r: &ray, t_min: f64, t_max: f64) -> bool {
        let (mut calc_min, mut calc_max) = (t_min, t_max);

        for i in 0..3 {
            // (1D_Point - Ray_Origin_At_same_dimension ) / Ray_Direction_At_same_direction = 1D_Point_where_ray_intersects_1D_Point
            // We take the minimum between 1D_min_AABB and 1D_max_AABB -> to make sure we can know we have a minimum for reference
            let inv = 1. / r.dir.v[i];
            let expected_min = (self.min.v[i] - r.origin.v[i]) * inv;
            let expected_max = (self.max.v[i] - r.origin.v[i]) * inv;
            let t0_actual_min = expected_min.min(expected_max);
            let t1_actual_max = expected_max.max(expected_min);

            // Then we tighten the t_value by getting them closer each time
            // Our initial ray_constraints (t_min, t_max) are constrained agains the new actual_mins
            
            calc_min = t0_actual_min.max(calc_min);
            calc_max = t1_actual_max.min(calc_max);
            
            // Then if we have crosse the boundaries by tightening, it means we are no no longer inside the AABB
            // We would be calculating overlap between boundaries outside the expected from AABB
            if(calc_max <= calc_min) 
                { return false }
        }
        true
    }

    pub fn hit_fast(&self, r: &ray, t_min: f64, t_max: f64) -> bool {
        let (mut calc_min, mut calc_max) = (t_min, t_max);
        for i in 0..3 {
            let inverse_dir = 1. / r.dir.v[i];
            let mut t0 = (self.min.v[i] - r.origin.v[i]) * inverse_dir;
            let mut t1 = (self.max.v[i] - r.origin.v[i]) * inverse_dir;
            
            if inverse_dir < 0. {
                let mid = t0;
                t0 = t1;
                t1 = mid;
            }

            calc_min = t0.max(calc_min); //if t0 > calc_min { t0 } else { calc_min };
            calc_max = t1.min(calc_max); //if t1 < calc_max { t1 } else { calc_max };
            if calc_max <= calc_min {return false}
            // tmax <= t_min
        } 

        true
    }

    pub fn hit_branchless(&self, r: &ray, t_min: f64, t_max: f64) -> bool {
        let (mut calc_min, mut calc_max) = (t_min, t_max);
        let inverse_dir = vec3::from(1./r.dir.v[0], 1./r.dir.v[1], 1./r.dir.v[2]);
        
        let mut t0 = (self.min.v[0] - r.origin.v[0]) * inverse_dir.v[0];
        let mut t1 = (self.max.v[0] - r.origin.v[0]) * inverse_dir.v[0];

        calc_min = t0.min(t1);
        calc_max = t0.max(t1);

        let mut t0 = (self.min.v[1] - r.origin.v[1]) * inverse_dir.v[1];
        let mut t1 = (self.max.v[1] - r.origin.v[1]) * inverse_dir.v[1];

        calc_min = t0.min(t1);
        calc_max = t0.max(t1);

        let mut t0 = (self.min.v[2] - r.origin.v[2]) * inverse_dir.v[2];
        let mut t1 = (self.max.v[2] - r.origin.v[2]) * inverse_dir.v[2];

        calc_min = t0.min(t1);
        calc_max = t0.max(t1);

        calc_max >= calc_min
    }
}

use crate::objects::hittable_list::*;
use std::sync::Arc;
use std::sync::Mutex;

unsafe impl Send for aabb {}
unsafe impl Sync for aabb {}
impl Hittable for aabb {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:& mut hit_record) -> bool {
        rec.iters += 1;
        self.hit_fast(r, t_min, t_max)
    }

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (true, aabb::from(self.min, self.max))
    }
}