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

use std::sync::Arc;
use std::sync::Mutex;
use crate::threadpool::*;
use std::thread;
use std::time::Duration;

pub fn Unoptimized() {
    let pool = ThreadPool::new(8);
    
    // Base Values
    let samples = 10;
    let depth = 5;

    let aspect_ratio = 16. / 9.;
    let image_width = 240;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let focal_length = 1.;
    
    let og = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.1;
    let cam = camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist);
    
    // New Materials
    let mut material_vec: Arc<Mutex<Vec<Arc<dyn Material>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut obj_vec: Arc<Mutex<Vec<Box<dyn Hittable>>>> = Arc::new(Mutex::new(Vec::new()));

    {
        let mut lcl_mat_vec = material_vec.lock().unwrap();
        lcl_mat_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.5,0.5,0.5)}));
        let mut lcl_obj_vec = obj_vec.lock().unwrap();
        lcl_obj_vec.push(Box::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., lcl_mat_vec[0].clone())));
    }

    for i in (-11..11) {
        for j in (-11..11) {
            let mut lcl_mat_arc = Arc::clone(&material_vec);
            let mut lcl_obj_arc = Arc::clone(&obj_vec);

            pool.execute(|| {
                let mat_rng = rand_f64();
                let center = point3::from(i as f64 + 0.9 * rand_f64(), 0.2, j as f64 + 0.9*rand_f64());

                let mut mat_push: Arc<dyn Material>;
                let mut obj_push: Arc<dyn Hittable>;

                if(center - point3::from(4.,0.2,0.)).length() > 0.9 {
                    if (mat_rng < 0.8) { // diffuse
                        let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));

                        mat_push = Arc::new(lambertian{albedo});

                    } else if mat_rng < 0.95 { // metal
                        let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                        let fuzz = rand_f64_r(0., 0.5);

                        mat_push = Arc::new(metal{albedo, fuzz});
                    } else { // glass
                        let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                        let index_refr = rand_f64_r(1., 2.);
                        let alpha = rand_f64_r(0., 0.5);

                        mat_push = Arc::new(dielectric{albedo, alpha, index_refr});
                    }
                    
                    let mut s: usize;
                    {
                        let lcl_mat_vec = lcl_mat_arc.lock().unwrap();
                        lcl_mat_vec.push(mat_push);
                        s = lcl_mat_vec.len();
                    }
                    {
                        let lcl_obj_vec = lcl_obj_arc.lock().unwrap();
                        lcl_obj_vec.push(Box::new(sphere::from_mat(center, 0.2, mat_push.clone())));
                    }
                }
            });
            

            //init_times.push(init_cost_timer.ms());
        }
    }
    {
        let mut lcl_mat_vec = material_vec.lock().unwrap();
        let mut lcl_obj_vec = obj_vec.lock().unwrap();

        lcl_mat_vec.push(Arc::new(dielectric{albedo: colorRGB::from(1., 1.,1.), alpha: 0., index_refr: 1.5}));
        lcl_obj_vec.push(Box::new(sphere::from_mat(point3::from(0., 1., 0.), 1., lcl_mat_vec[lcl_mat_vec.len()-1].clone())));

        lcl_mat_vec.push(Arc::new(metal{albedo: colorRGB::from(0.7, 0.6, 0.5), fuzz: 0.}));
        lcl_obj_vec.push(Box::new(sphere::from_mat(point3::from(4., 1., 0.), 1., lcl_mat_vec[lcl_mat_vec.len()-1].clone())));

        lcl_mat_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.7,0.6,0.5)}));
        lcl_obj_vec.push(Box::new(sphere::from_mat(point3::from(-4., 1., 0.), 1., lcl_mat_vec[lcl_mat_vec.len()-1].clone())));
    }
    //let total_init_time: f32 = init_times.iter().sum();
    //eprintln!("Avg Creat Cost: {}ms\nTotal Cost: {}ms", total_init_time / init_times.len() as f32, total_init_time);

    thread::sleep(Duration::from_millis(100));

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    // Throw a ray at every pixel
    let mut lcl_obj_vec = obj_vec.lock().unwrap();

    for i in (0..(image_height)).rev() {
        let float_i: f64 = (i).into();
        //eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..image_width) {
            //ray_cost_timer.restart();
            
            let float_j: f64 = (j).into();
            let mut pixel = colorRGB::new();
            
            for s in (0..samples) {
                //let x_sum = (s as f64).sqrt()
                let u = (float_j + rand_f64()) / (image_width as f64 - 1.);
                let v = (float_i + rand_f64()) / (image_height as f64 - 1.);
                let r = cam.focus_ray(u, v);
                pixel = pixel + ray_hits(&r, &lcl_obj_vec, depth);
            }
            pixel.write_color(samples as f64);

            //ray_times.push(ray_cost_timer.ms());
        }
    }

    //let total_ray_cost: f32 = ray_times.iter().sum();
    //eprintln!("Avg Ray Cost: {}ms\nTotal Ray Cost: {}ms", total_ray_cost / ray_times.len() as f32, total_ray_cost);
    eprintln!("\nDone with {} samples per pixel, Unoptimized Single Thread", samples);  
}