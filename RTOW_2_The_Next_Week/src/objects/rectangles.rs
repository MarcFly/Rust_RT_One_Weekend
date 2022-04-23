use crate::materials::*;
use std::sync::Arc;
use crate::rtow_math::vec3::*;
use crate::rtow_math::ray::*;
use crate::materials::Material;
use crate::materials::Default;
use crate::objects::hit::*;
use crate::objects::aabb::*;
use crate::rtow_math::vec2::*;

pub struct xy_rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat: Arc<dyn Material>,
}

impl xy_rect {
    //pub fn new() -> xy_rect {xy_rect{x0: 0., x1: 0., y0: 0., y1:0., k:0., mat: Arc::new}}
    pub fn from(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> xy_rect {
        xy_rect{x0, x1, y0, y1, k, mat}
    }
}

unsafe impl Sync for xy_rect{}
unsafe impl Send for xy_rect{}

impl Hittable for xy_rect {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:&mut hit_record) -> bool {
        // Ray depth where it should hit the plane containing the rectangle
        // As it is a axis aligned rectange to the z plane
        let t = (self.k - r.origin.v[2]) / r.dir.v[2];

        let x = r.origin.v[0] + r.dir.v[0] * t;
        let y = r.origin.v[1] + r.dir.v[1] * t;

        if (x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1) {return false;}

        rec.uv.v[0] = (x-self.x0) / (self.x1-self.x0);
        rec.uv.v[1] = (y-self.y0) / (self.y1-self.y0);

        rec.t = t;

        let out_normal = vec3::from(0.,0.,1.);
        rec.set_face_normal(r);
        rec.mat = Arc::clone(&self.mat);
        rec.p = r.at(t);

        true
    }
    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (
            true,
            aabb::from(point3::from(self.x0, self.y0, self.k-0.0001), point3::from(self.x1, self.y1, self.k+0.0001))
        )
    }

}