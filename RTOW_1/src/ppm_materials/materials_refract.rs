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

pub fn output_materials_refract() {
    let samples = 100;
    let depth = 50;

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let focal_length = 1.;
    
    let cam = camera::from(aspect_ratio, 2. * aspect_ratio as f64, focal_length);
    let cam = camera::from_fov(aspect_ratio, 90.);
    //let cam = camera::new();
    
    // New Materials
    let mat_ground = Rc::new(lambertian{albedo: colorRGB::from(0.8, 0.8, 0.0)});
    let mat_center = Rc::new(lambertian{albedo: colorRGB::from(0.1, 0.2, 0.5)}); //, alpha:0.2, index_refr: 1.5});
    let mat_left = Rc::new(dielectric{albedo: colorRGB::from(0.8, 0.4, 0.1), alpha: 1.0, index_refr: 1.5});
    let mat_right = Rc::new(metal{albedo: colorRGB::from(0.8, 0.6, 0.2), fuzz: 0.});
    
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(sphere::from_mat(point3::from(0., 0., -1.), 0.5, mat_center.clone())));
    //hittables.push(Box::new(sphere::from_mat(point3::from(0., 0., -1.), -0.4, mat_center.clone())));
    hittables.push(Box::new(sphere::from_mat(point3::from(0., -100.5, -1.), 100., mat_ground)));
    hittables.push(Box::new(sphere::from_mat(point3::from(-1., 0., -1.), 0.5, mat_left.clone())));
    hittables.push(Box::new(sphere::from_mat(point3::from(-1., 0., -1.), -0.4, mat_left.clone())));
    hittables.push(Box::new(sphere::from_mat(point3::from(1., 0., -1.), 0.5, mat_right)));
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
                let r = cam.ray(u, v);
                pixel = pixel + ray_hits(&r, &hittables, depth);
            }
            pixel.write_color(samples as f64);
        }
    }
    eprintln!("\nDone with {} samples per pixel", samples);

}