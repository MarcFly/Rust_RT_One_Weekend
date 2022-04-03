use crate::rtow_math::vec3::*;
use crate::rtow_math::ray::*;
use crate::objects::hit::*;

pub enum LightType {
    Point,
    Directional,
    Spotlight,
}

pub struct light {
    pub center: point3,
    pub intensity: f64,
    pub color: colorRGB,
    pub direction: vec3,
    pub ltype: LightType,
}

impl light {
    pub fn new_point(center: point3, intensity: f64, color: colorRGB) -> light {
        light { center, intensity, color, direction: vec3::new(), ltype: LightType::Point }
    }

    pub fn new_direction(direction: vec3, intensity: f64, color: colorRGB) -> light {
        light { center: point3::new(), intensity, color, direction, ltype: LightType::Directional }
    }

    pub fn new_spotlight(center: point3, direction: vec3, intensity: f64, color: colorRGB) -> light {
        light { center, intensity, color, direction, ltype: LightType::Spotlight }
    }
    
    pub fn hit_point(&self,r: &ray, t_min: f64, t_max: f64, rec:& mut hit_record ) -> bool {
        let origin_center = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = origin_center.dot(&r.dir);
        let c = origin_center.dot(&origin_center) - self.intensity*self.intensity;
        
        let mut discriminant = half_b*half_b - a*c;
        //discriminant = check_discriminant(discriminant, half_b, a);
        if (discriminant < 0.) {return false}
        
        let sq_discr = discriminant.sqrt();
        let mut root = (-half_b - sq_discr) / a;
        if(root < t_min || root > t_max) {
            root = (-half_b +sq_discr) / a;
            if(root < t_min || t_max < root) {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.n = (rec.p - self.center) / self.intensity; // This is bad, only returns normal pointing outwards
        // What if we need to differentiate between from and back face!
        rec.set_face_normal(r);
        //rec.mat = Arc::clone(&self.mat);

        true
    }
}

use crate::objects::aabb::*;

use crate::objects::hittable_list::*;
use std::sync::Arc;
use std::sync::Mutex;

impl Hittable for light {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:& mut hit_record) -> bool {
        
        match self.ltype {
            LightType::Point => {
                self.hit_point(r, t_min, t_max, rec)
            },
            _ => false
        }
    }

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (false, aabb::new()) // Will have to be solved by type...
    }
}