use crate::materials::*;
use crate::rtow_math::{
    vec3::*, 
    ray::*,
    rng::*,
    camera::*,
    defines::*,
};

pub trait Emissive {
    
}

pub struct Diffuse_Emissive {
    pub albedo: colorRGB,
    pub tex: Arc<dyn Texture>,
}

impl Material for Diffuse_Emissive {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        false
    }

    fn scatter_tex(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        if v.is_nan() {
            let t = true;
        }
        self.tex.value(u,v,p) * self.albedo
    }
}