#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_math;
use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*
};

use simple_stopwatch::Stopwatch;

fn main() {
    let mut timer = Stopwatch::start_new();
    Chapters1_7();
    Chapters8_12();
    final_render_ch13();



    eprintln!("Took {} ms", timer.ms());
}

mod ppm_tests;
use crate::ppm_tests::{
    antialiasing::*,
    base_image::*,
    camera_rays::*,
    hittable_list::*,
    sphere_1::*,
    sphere_normals::*,
};

fn Chapters1_7(){
    //output_ppm();
    //output_ppm_vec3();
    //output_ppm_cam();
    //output_ppm_sphere();
    //output_ppm_s_normals();
    //object_list_test();
    //output_multisample();
}

mod ppm_materials;
use crate::ppm_materials::diffuse::*;
use crate::ppm_materials::materials_1::*;
use crate::ppm_materials::materials_refract::*;
use crate::ppm_materials::camera_move::*;
use crate::ppm_materials::camera_focus::*;

mod materials;

fn Chapters8_12() {
    //output_diffuse();
    //output_materials();
    //output_materials_refract();
    //output_cam_move();
    //output_cam_focus();
}


use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*,
    rng::*,
    camera::*,
    defines::*,
};

use crate::materials::*;

/// Largers cube in sphere is of size 1./(3.0.sqrt())
// const side: f64 = 1. / (3.0 as f64).sqrt();
/// Then add 1. - (1./3.0.sqrt) / 2. to put the point inside sphere 100%
// const side_sum: f64 = (1. - side) /2.;



fn ray_hits(r: &ray, obj: &Vec<Box<dyn Hittable>>, depth: i32) ->  colorRGB {
    if(depth < 1) {return colorRGB::new()}

    let next_depth = depth -1;
    
    let mut rec = hit_record::new();
    
    // From Antialiasing
    // Made recursive, with depth limit now
    // Will start generating rays around in random_in_sphere
    // Setting t_min at 0.001 increases light MASSIVELY, why?
    if hit_list(obj, 0.0001, std::f64::INFINITY, &mut rec, r) {
        let mut scattered = ray::new();
        let mut attenuation = colorRGB::new();
        if(rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered)){
            return ray_hits(&scattered, obj, next_depth) * attenuation;
        }
        return colorRGB::from(0.5,0.5,0.5);
    }

    let unit_dir = r.dir.unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    colorRGB::from(1.,1.,1.)*(1.0 - t) + colorRGB::from(0.5, 0.7, 1.0) * t
}

use std::rc::Rc;

fn final_render_ch13() {
    let samples = 100;
    let depth = 10;

    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let focal_length = 1.;
    
    let og = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.1;
    let cam = camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist);
    
    // New Materials
    let mut material_vec: Vec<Rc<dyn Material>> = Vec::new();
    let mut obj_vec: Vec<Box<dyn Hittable>> = Vec::new();

    material_vec.push(Rc::new(lambertian{albedo: colorRGB::from(0.5,0.5,0.5)}));
    obj_vec.push(Box::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., material_vec[0].clone())));
    
    for i in (-11..11) {
        for j in (-11..11) {
            let mat_rng = rand_f64();
            let center = point3::from(i as f64 + 0.9 * rand_f64(), 0.2, j as f64 + 0.9*rand_f64());
            if(center - point3::from(4.,0.2,0.)).length() > 0.9 {
                if (mat_rng < 0.8) { // diffuse
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    material_vec.push(Rc::new(lambertian{albedo}));
                    let s = material_vec.len();
                    obj_vec.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else if mat_rng < 0.95 { // metal
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let fuzz = rand_f64_r(0., 0.5);
                    material_vec.push(Rc::new(metal{albedo, fuzz}));
                    let s = material_vec.len();
                    obj_vec.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else { // glass
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let index_refr = rand_f64_r(1., 2.);
                    let alpha = rand_f64_r(0., 0.5);
                    material_vec.push(Rc::new(dielectric{albedo, alpha, index_refr}));
                    let s = material_vec.len();
                    obj_vec.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                }
            }
        }
    }

    material_vec.push(Rc::new(dielectric{albedo: colorRGB::from(1., 1.,1.), alpha: 0., index_refr: 1.5}));
    obj_vec.push(Box::new(sphere::from_mat(point3::from(0., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Rc::new(metal{albedo: colorRGB::from(0.7, 0.6, 0.5), fuzz: 0.}));
    obj_vec.push(Box::new(sphere::from_mat(point3::from(4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Rc::new(lambertian{albedo: colorRGB::from(0.7,0.6,0.5)}));
    obj_vec.push(Box::new(sphere::from_mat(point3::from(-4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    // Throw a ray at every pixel
    for i in (0..(image_height)).rev() {
        let float_i: f64 = (i).into();
        //eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..image_width) {
            let float_j: f64 = (j).into();
            let mut pixel = colorRGB::new();
            for s in (0..samples) {
                //let x_sum = (s as f64).sqrt()
                let u = (float_j + rand_f64()) / (image_width as f64 - 1.);
                let v = (float_i + rand_f64()) / (image_height as f64 - 1.);
                let r = cam.focus_ray(u, v);
                pixel = pixel + ray_hits(&r, &obj_vec, depth);
            }
            pixel.write_color(samples as f64);
        }
    }
    eprintln!("\nDone with {} samples per pixel", samples);
}