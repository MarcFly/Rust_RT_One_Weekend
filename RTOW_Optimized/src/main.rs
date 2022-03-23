#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_math;
mod materials;

mod optimized;

mod threadpool;
mod taskrunner;
use crate::taskrunner::*;
use crate::threadpool::*;
use std::sync::mpsc;

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
    let mut attenuation = colorRGB::new();
    if hit_list(&*obj, 0.0001, std::f64::INFINITY, &mut rec, r) {
        let mut scattered = ray::new();
        //let mut attenuation = colorRGB::new();
        unsafe{
        if(Material::scatter(&*rec.mat, r, &rec, &mut attenuation, &mut scattered)){
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
static depth: i32 = 10;

enum Pixel {
    RGB(usize, colorRGB),
}

fn main() {
    let mut timer = Stopwatch::start_new();

    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.1;
    let cam = camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist);
    
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.5,0.5,0.5)}));
    hittables.push(Box::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., Arc::clone(& material_vec[0]))));
    
    for i in (-11..11) {
        for j in (-11..11) {
            let mat_rng = rand_f64();
            let center = point3::from(i as f64 + 0.9 * rand_f64(), 0.2, j as f64 + 0.9*rand_f64());
            if(center - point3::from(4.,0.2,0.)).length() > 0.9 {
                if (mat_rng < 0.8) { // diffuse
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    material_vec.push(Arc::new(lambertian{albedo}));
                    let s = material_vec.len();
                    hittables.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else if mat_rng < 0.95 { // metal
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let fuzz = rand_f64_r(0., 0.5);
                    material_vec.push(Arc::new(metal{albedo, fuzz}));
                    let s = material_vec.len();
                    hittables.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else { // glass
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let index_refr = rand_f64_r(1., 2.);
                    let alpha = rand_f64_r(0., 0.5);
                    material_vec.push(Arc::new(dielectric{albedo, alpha, index_refr}));
                    let s = material_vec.len();
                    hittables.push(Box::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                }
            }
        }
    }
    material_vec.push(Arc::new(dielectric{albedo: colorRGB::from(1., 1.,1.), alpha: 0., index_refr: 1.5}));
    hittables.push(Box::new(sphere::from_mat(point3::from(0., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Arc::new(metal{albedo: colorRGB::from(0.7, 0.6, 0.5), fuzz: 0.}));
    hittables.push(Box::new(sphere::from_mat(point3::from(4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.7,0.6,0.5)}));
    hittables.push(Box::new(sphere::from_mat(point3::from(-4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    let mut arc_cols: Arc<Mutex<Box<Vec<colorRGB>>>> = Arc::new(Mutex::new(Box::new(Vec::new()))); // VecPoint));
    {
        let mut vec = arc_cols.lock().unwrap();
        //for i in 0..image_width * image_height as usize {
        //    vec.push(Arc::new(Mutex::new(colorRGB::new())));
        //}
        vec.resize(image_width * image_height as usize, colorRGB::new());
    }
    eprintln!("Finished creating individual ArcMutexColorRGB at {} ms", timer.ms());
    //let num_thread = std::thread::available_parallelism().unwrap().get();
    //let mut sender: mpsc::Sender<Pixel>;
    //let mut receiver: mpsc::Receiver<Pixel>;
    let (sender, receiver) = mpsc::channel();

    let arc_hit = Arc::new(hittables);
    {
        let mut tp =  Runner::new(12);
        let mut v_smth = arc_cols.lock().unwrap();
        // Throw a ray at every pixel
        for i in (0..(image_height)).rev() {
            let float_i: f64 = (i).into();
            //eprintln!("\rScanlines Remaining: {}\r", i);
            for j in (0..image_width) {
                let float_j: f64 = j as f64;
                let hit_arc = Arc::clone(&arc_hit);
                //let mut mut_col = &mut color_arr[(i*j as i32) as usize];
                //let mut curr_pixel: *mut colorRGB = &mut VecPoint[i as usize * j as usize] as * mut colorRGB;
                let idx = (image_width * (image_height - i - 1 ) as usize + j ); //as usize;
                //let mut curr_pixel = Arc::clone(&(v_smth[idx]));
                
                let sender_cpy = sender.clone();
                tp.add_task(move || {
                    let mut pixel = colorRGB::new();
                    for s in (0..samples) {
                        //let x_sum = (s as f64).sqrt()
                        let u = (float_j + rand_f64()) / (iw_f64 - 1.);
                        let v = (float_i + rand_f64()) / (ih_f64 - 1.);
                        let r = cam.focus_ray(u, v);
                        pixel = pixel + ray_hits(&r, Arc::clone(&hit_arc), depth);
                    }

                    sender_cpy.send(Pixel::RGB(idx, pixel));
                    //pixel.write_col_to(curr_pixel, idx);
                });
                
            }
        }

        let mut num_pixels = image_width * image_height as usize;
        //let mut vec = arc_cols.lock().unwrap();
        while (num_pixels > 0) {
            match receiver.recv().unwrap() {
                Pixel::RGB(idx, col) => {
                    v_smth[idx] = col;
                    num_pixels -= 1;
                },
                _ => ()
            }
        }
        //{
        //    eprintln!("Finished sending tasks at {} ms", timer.ms());
        //    //tp.ocupancy();
        //    eprintln!("Start thread wait at {} ms", timer.ms());
        //    tp.wait_all();
        //    eprintln!("Waited threads, finished at {} ms", timer.ms());
        //}
    }
    eprintln!("Tasks finished running at {} ms", timer.ms());
    {
        let mut vec = arc_cols.lock().unwrap();
        for i in (0..vec.len()) {
            vec[i].write_color(samples as f64);
        }
    }

    eprintln!("Took {} ms", timer.ms());
}