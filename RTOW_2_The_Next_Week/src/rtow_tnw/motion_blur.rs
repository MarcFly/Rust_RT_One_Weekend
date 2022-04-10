use crate::taskrunner::*;
use crate::threadpool::*;
use std::sync::mpsc;

use simple_stopwatch::Stopwatch;

use crate::objects::{
    hit::*,
    sphere::*,
    lights::*,
};

use crate::rtow_math::{
    vec3::*, 
    ray::*,
    rng::*,
    camera::*,
    defines::*,
};
use crate::materials::*;
use crate::rtow_tnw::*;

use std::sync::*;

fn light_hits(r: &ray, lights: Arc<Vec<light>>, obj: Arc<hittable_list>) -> colorRGB {
    // Direct ray to find first object from camera
    // Then from point hit to all lights for each contribution
    // if blocked, no contribution
    let mut rec = hit_record::new();
    obj.hit(0.0001, std::f64::INFINITY, &mut rec, r);

    let mut color = colorRGB::new();
    let l_slice = lights.iter().as_slice();
    for l in l_slice {
        let new_r = ray::from_t(rec.p, l.center - rec.p, r.time);
        if !obj.hit(0.0001, std::f64::INFINITY, &mut rec, &new_r) {
            color = color +  l.color * l.intensity * ( 1. / (4. * 3.14 * new_r.dir.length_squared())); // No light loss for now
        }
    };

    color
}

fn ray_hits(r: &ray, obj: Arc<hittable_list>, depth_: i32) ->  colorRGB {
    if(depth_ < 1) {return colorRGB::new()}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    
    // From Antialiasing
    // Made recursive, with depth limit now
    // Will start generating rays around in random_in_sphere
    // Setting t_min at 0.001 increases light MASSIVELY, why?
    let mut attenuation = colorRGB::new();
    if obj.hit(0.0001, std::f64::INFINITY, &mut rec, r) {
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
enum Pixel {
    RGB(usize, colorRGB),
}

pub fn render() {
    let mut timer = Stopwatch::start_new();

    let (cam, image_width, image_height) = base_cam();
    let (iw_f64, ih_f64) = (image_width as f64, image_height as f64);
    // SETUP Lights for direct shadow rays

    let mut lights = setup_direct_lights();
    let arc_lights = Arc::new(lights);

    // SETUP Objects and materials 
    
    let (mut hittables, mut material_vec) = setup_objects();

    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    //let mut arc_cols: Arc<Mutex<Box<Vec<colorRGB>>>> = Arc::new(Mutex::new(Box::new(Vec::new()))); // VecPoint));
    let mut arc_cols: Arc<Mutex<Box<Vec<Arc<Mutex<colorRGB>>>>>> = Arc::new(Mutex::new(Box::new(Vec::new())));
    {
        let mut vec = arc_cols.lock().unwrap();
        for i in 0..image_width * image_height {
            vec.push(Arc::new(Mutex::new(colorRGB::new())));
        }
        //vec.resize(image_width * image_height as usize, colorRGB::new());
    }
    eprintln!("Finished creating individual ArcMutexColorRGB at {} ms", timer.ms());
    //let num_thread = std::thread::available_parallelism().unwrap().get();
    //let mut sender: mpsc::Sender<Pixel>;
    //let mut receiver: mpsc::Receiver<Pixel>;
    //let (sender, receiver) = mpsc::channel();

    let arc_hit = Arc::new(hittables);
    {
        let mut tp =  Runner::new(24);
        let mut v_smth = arc_cols.lock().unwrap();
        // Throw a ray at every pixel
        for i in (0..(image_height)).rev() {
            let float_i: f64 = (i).into();
            //eprintln!("\rScanlines Remaining: {}\r", i);
            for j in (0..image_width) {
                let float_j: f64 = j as f64;
                
                let hit_arc = Arc::clone(&arc_hit);
                let idx = (image_width * (image_height - i - 1 ) + j ) as usize; //as usize;
                
                let light_arc = Arc::clone(&arc_lights);

                //let sender_cpy = sender.clone();
                let curr_pixel = Arc::clone(&v_smth[idx]);

                tp.add_task(move || {
                    let mut pixel = colorRGB::new();
                    for s in (0..samples) {
                        //let x_sum = (s as f64).sqrt()
                        let u = (float_j + rand_f64()) / (iw_f64 - 1.);
                        let v = (float_i + rand_f64()) / (ih_f64 - 1.);
                        let r = cam.focus_time_ray(u, v);
                        let ambient_indirect = ray_hits(&r, Arc::clone(&hit_arc), depth);
                        let lights_direct = light_hits(&r, Arc::clone(&light_arc), Arc::clone(&hit_arc));
                        pixel = pixel + ambient_indirect + lights_direct;
                    }                   
                    //let u = (float_j) / (iw_f64 - 1.);
                    //let v = (float_i) / (ih_f64 - 1.);
                    //let r = cam.focus_ray(u, v);
                    //pixel = pixel + light_hits(&r, Arc::clone(&light_arc), Arc::clone(&hit_arc)) * (samples as f64);

                    //sender_cpy.send(Pixel::RGB(idx, pixel));
                    pixel.write_col_to(curr_pixel, idx);
                });
                
            }
        }

        let mut num_pixels = image_width * image_height;
        //let mut vec = arc_cols.lock().unwrap();
        //while (num_pixels > 0) {
        //    match receiver.recv().unwrap() {
        //        Pixel::RGB(idx, col) => {
        //            v_smth[idx] = col;
        //            num_pixels -= 1;
        //        },
        //        _ => ()
        //    }
        //}
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
            vec[i].lock().unwrap().write_color(samples as f64);
        }
    }

    eprintln!("Took {} ms", timer.ms());
}