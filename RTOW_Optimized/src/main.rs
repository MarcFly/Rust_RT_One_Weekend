#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_math;
mod materials;

mod optimized;

mod threadpool;
use crate::threadpool::*;

use simple_stopwatch::Stopwatch;

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
use std::sync::*;

fn ray_hits(r: &ray, obj: Arc<Vec<Box<dyn Hittable>>>, depth_: i32) ->  colorRGB {
    if(depth_ < 1) {return colorRGB::new()}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    
    // From Antialiasing
    // Made recursive, with depth limit now
    // Will start generating rays around in random_in_sphere
    // Setting t_min at 0.001 increases light MASSIVELY, why?
    if hit_list(&*obj, 0.0001, std::f64::INFINITY, &mut rec, r) {
        let mut scattered = ray::new();
        let mut attenuation = colorRGB::new();
        unsafe{
        if(Material::scatter(&***(rec.mat), r, &rec, &mut attenuation, &mut scattered)){
            return ray_hits(&scattered, obj, next_depth) * attenuation;
        }
    }
        return colorRGB::from(0.5,0.5,0.5);
    }

    let unit_dir = r.dir.unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    colorRGB::from(1.,1.,1.)*(1.0 - t) + colorRGB::from(0.5, 0.7, 1.0) * t
}

static samples: i32 = 100;
static depth: i32 = 50;

fn main() {
    let mut timer = Stopwatch::start_new();

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(3.,3.,2.);
    let lookat = point3::from(0.,0.,-1.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 2.;
    let cam = camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist);
    
    // New Materials
    let mat_ground = lambertian{albedo: colorRGB::from(0.8, 0.8, 0.0)};
    let mat_center = lambertian{albedo: colorRGB::from(0.1, 0.2, 0.5)}; //, alpha:0.2, index_refr: 1.5});
    let mat_left = dielectric{albedo: colorRGB::from(0.8, 0.4, 0.1), alpha: 1.0, index_refr: 1.5};
    let mat_left_2 = dielectric{albedo: colorRGB::from(0.8, 0.4, 0.1), alpha: 1.0, index_refr: 1.5};
    let mat_right = metal{albedo: colorRGB::from(0.8, 0.6, 0.2), fuzz: 0.};
    
    let mut hittables:Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(sphere::from_mat(point3::from(0., 0., -1.), 0.5, Box::new(&mat_center))));
    //hittables.push(Box::new(sphere::from_mat(point3::from(0., 0., -1.), -0.4, mat_center.clone())));
    hittables.push(Box::new(sphere::from_mat(point3::from(0., -100.5, -1.), 100., Box::new(&mat_ground))));
    hittables.push(Box::new(sphere::from_mat(point3::from(-1., 0., -1.), 0.5, Box::new(&mat_left))));
    hittables.push(Box::new(sphere::from_mat(point3::from(-1., 0., -1.), -0.4, Box::new(&mat_left_2))));
    hittables.push(Box::new(sphere::from_mat(point3::from(1., 0., -1.), 0.5, Box::new(&mat_right))));

    let mut color_vec: Vec<colorRGB> = Vec::new();
    //let mut color_arr: [colorRGB; 400*400] = [colorRGB::new(); 400*400 ];
    color_vec.resize(image_width * image_height as usize, colorRGB::new());

    println!("P3\n{} {}\n255\n", image_width, image_height);

    //let mut VecPoint: Box<Vec<colorRGB>> = Box::new(Vec::new());
    //VecPoint.resize(image_width * image_height as usize, colorRGB::new());
    
    let mut arc_cols: Arc<Mutex<Box<Vec<colorRGB>>>> = Arc::new(Mutex::new(Box::new(Vec::new()))); // VecPoint));
    {
        arc_cols.lock().unwrap().resize(image_width * image_height as usize, colorRGB::new());
    }

    let num_thread = std::thread::available_parallelism().unwrap().get();

    let arc_hit = Arc::new(hittables);
    {
        let tp =  ThreadPool::new(num_thread);

        // Throw a ray at every pixel
        for i in (0..(image_height)).rev() {
            let float_i: f64 = (i).into();
            //eprintln!("\rScanlines Remaining: {}\r", i);
            for j in (0..image_width) {
                let float_j: f64 = j as f64;
                let hit_arc = Arc::clone(&arc_hit);
                //let mut mut_col = &mut color_arr[(i*j as i32) as usize];
                //let mut curr_pixel: *mut colorRGB = &mut VecPoint[i as usize * j as usize] as * mut colorRGB;
                let mut curr_pixel = Arc::clone(&arc_cols);
                let idx = (image_width * (image_height - i - 1 ) as usize + j ); //as usize;
                tp.execute(move || {
                    let mut pixel = colorRGB::new();
                    for s in (0..samples) {
                        //let x_sum = (s as f64).sqrt()
                        let u = (float_j + rand_f64()) / (iw_f64 - 1.);
                        let v = (float_i + rand_f64()) / (ih_f64 - 1.);
                        let r = cam.focus_ray(u, v);
                        pixel = pixel + ray_hits(&r, Arc::clone(&hit_arc), depth);
                    }
                    pixel.write_col_to(curr_pixel, idx);
                });
                
            }
        }
    }
    
    {
        let mut vec = arc_cols.lock().unwrap();

        for i in (0..vec.len()) {
            vec[i].write_color(samples as f64);
        }
    }

    eprintln!("Took {} ms", timer.ms());
}