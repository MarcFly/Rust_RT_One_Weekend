use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*,
    rng::*,
    camera::*,
    defines::*,
};

pub trait Material {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool;
}

pub struct Default {}

impl Material for Default {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        false
    }
}

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
        *scatter = ray::from(&rec.p, &scatter_dir);
        *attenuation = self.albedo;
        true
    }

}

/// Metal Materials
pub struct metal {
    pub albedo: colorRGB,
}

impl Material for metal {
    fn scatter(&self, r: &ray, rec: &hit_record, attenuation: &mut colorRGB, scatter: &mut ray) -> bool {
        let mut scatter_dir = r.dir.reflect(&rec.n).unit_vec();

        *scatter = ray::from(&rec.p, &scatter_dir);
        *attenuation = self.albedo;
        
        (scatter.dir.dot(&rec.n) > 0.)
    }
}