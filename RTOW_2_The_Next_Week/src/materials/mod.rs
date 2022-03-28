use crate::rtow_math::{
    vec3::*, 
    ray::*,
    rng::*,
    camera::*,
    defines::*,
};

use crate::objects::{
    hit::*,
    sphere::*,

};

pub trait Material {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool;
}

pub struct Default {}
pub static def_material: Default = Default{};

impl Material for Default {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        false
    }
}

//impl Send for dyn Material {}
//impl Sync for dyn Material {}

/// Lambertian Materials
pub struct lambertian {
    pub albedo: colorRGB,
}

impl Material for lambertian {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        let mut scatter_dir = rec.n + random_in_sphere();
        if(scatter_dir.near_zero()) {
            scatter_dir = rec.n;
        }
        *scatter = ray::from(rec.p, scatter_dir);
        *attenuation = self.albedo;
        true
    }

}

/// Metal Materials
pub struct metal {
    pub albedo: colorRGB,
    pub fuzz: f64,
}

impl Material for metal {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        let mut scatter_dir = r.dir.unit_vec().reflect(&rec.n).unit_vec();
        scatter_dir = (scatter_dir + random_in_sphere() * self.fuzz).unit_vec(); // Fuzzy reflections   

        *scatter = ray::from(rec.p, scatter_dir);
        *attenuation = self.albedo;
        
        (scatter.dir.dot(&rec.n) > 0.)
    }
}

/// Dielectric Materials
pub struct dielectric {
    pub albedo: colorRGB,
    pub alpha: f64,
    pub index_refr: f64,
}

impl dielectric {
    pub fn new() -> dielectric {
        dielectric { 
            albedo: colorRGB::from(0.,0.,0.), 
            alpha: 0., 
            index_refr: 0. }
    }

    pub fn reflectance(cos: f64, ir: f64) -> f64 {
        let mut r0 = (1. - ir) / (1. + ir);
        r0 = r0*r0;
        r0 + (1.-r0) * ((1.-cos).powf(5.))
    }
}

impl Material for dielectric {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        *attenuation = self.albedo * self.alpha;
        *attenuation = (colorRGB::from(1.,1.,1.) + self.albedo*self.alpha);
        attenuation.clamp(0.,1.);
        let ratio = if rec.front_face { 1. / self.index_refr} else { self.index_refr };
        let unit = r.dir.unit_vec();
        
        let cos = (unit * -1.).dot(&rec.n).min(1.);
        let sin = (1. - cos * cos).sqrt();
        let cant_refract = ratio * sin > 1.;
        let mut refracted = if (cant_refract || dielectric::reflectance(cos, ratio) > rand_f64())  {
            unit.reflect(&rec.n)
        } else {
            unit.refract(&rec.n, ratio)
        };

        *scatter = ray::from(rec.p, refracted);
        true
    }
}