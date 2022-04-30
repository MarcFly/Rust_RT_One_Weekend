use crate::taskrunner::*;
use crate::threadpool::*;
use std::sync::mpsc;

use crate::materials::textures::*;

use simple_stopwatch::Stopwatch;

use crate::objects::{
    hit::*,
    sphere::*,
    lights::*,
    hittable_list::*,
};

use crate::rtow_math::{
    vec3::*, 
    ray::*,
    rng::*,
    camera::*,
    defines::*,
};
use crate::materials::*;
use std::sync::*;

fn light_hits(r: &ray, lights: Arc<Vec<light>>, obj: Arc<hittable_list>) -> colorRGB {
    // Direct ray to find first object from camera
    // Then from point hit to all lights for each contribution
    // if blocked, no contribution
    let mut rec = hit_record::new();
    obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r);

    let mut color = colorRGB::new();
    let l_slice = lights.iter().as_slice();
    for l in l_slice {
        let new_r = ray::from_t(rec.p, l.center - rec.p, r.time);
        if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, &new_r) {
            color = color +  l.color * l.intensity * ( 1. / (4. * 3.14 * new_r.dir.length_squared())); // No light loss for now
        }
    };

    color
}

fn ray_hits(r: &ray, obj: Arc<hittable_list>, depth_: i32, debug_iter_vec: Arc<Mutex<Vec<i32>>>, bg_col: colorRGB) ->  colorRGB {
    if(depth_ < 1) {return colorRGB::new()}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    let mut attenuation = colorRGB::new();

    if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r) {
        debug_iter_vec.lock().unwrap().push(rec.iters);
        return bg_col;
    }
    if rec.uv.v[0].is_nan() || rec.uv.v[1].is_nan() {
        let mut test = false;
        test = true;
    }
    let mut scattered = ray::new();
    let emitted = rec.mat.emitted(rec.uv.v[0], rec.uv.v[1], &rec.p);

    //let mut attenuation = colorRGB::new();
    //let res1 = rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered);
    //let mut attenuation2 = colorRGB::new();
    //let mut scattered2 = ray::new();
    //let mut rec2 = rec.clone();
    //let res2 = unsafe { Material::scatter_tex( &*rec2.mat, r, &rec2, &mut attenuation2, &mut scattered2) };
    //if res1 != res2 || attenuation != attenuation2 || scattered2 != scattered {
    //    panic!("What the fuck???")
    //}
    if !rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered) {
        debug_iter_vec.lock().unwrap().push(rec.iters);
        return emitted;
    }

    //unsafe {
    //    if !Material::scatter_tex( &*rec.mat, r, &rec, &mut attenuation, &mut scattered) {
    //        debug_iter_vec.lock().unwrap().push(rec.iters);
    //        return emitted;
    //    }
    //}

    debug_iter_vec.lock().unwrap().push(rec.iters);
    emitted + ray_hits(&scattered, obj, next_depth, Arc::clone(&debug_iter_vec), bg_col) * attenuation
}

enum Pixel {
    RGB(usize, colorRGB),
}

use crate::rtow_tnw::*;

pub fn render() {
    let mut timer = Stopwatch::start_new();

    let (cam, image_width, image_height) = cam_scene6_cornellbox();
    let (iw_f64, ih_f64) = (image_width as f64, image_height as f64);
    // SETUP Lights for direct shadow rays

    let mut lights = setup_direct_lights();
    let arc_lights = Arc::new(lights);

    // SETUP Objects and materials 
    
    let (mut hittables, mut material_vec) = obj_scene7_CornellBox_volumes();

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
    let bg_col = colorRGB::new();
    let arc_hit = Arc::new(hittables);
    let mut arc_iters: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    {
        let mut tp = Runner::new(8);
        let mut v_smth = arc_cols.lock().unwrap();

        let mut i = image_height;
        let mut j = 0;
        let mut iter = v_smth.iter_mut();
        loop {

            match iter.next() {
                Some(v) =>{
                    let hit_arc = Arc::clone(&arc_hit);
                    let light_arc = Arc::clone(&arc_lights);
                    let curr_pixel = Arc::clone(v);
                    let clone_iters = Arc::clone(&arc_iters);

                    tp.add_task(move || {
                        let mut pixel = colorRGB::new();
                        for s in (0..samples) {
                            //let x_sum = (s as f64).sqrt()
                            let u = (j as f64 + rand_f64()) / (iw_f64 - 1.);
                            let v = (i as f64 + rand_f64()) / (ih_f64 - 1.);
                            let r = cam.focus_time_ray(u, v);
                            let ambient_indirect = ray_hits(&r, Arc::clone(&hit_arc), depth, Arc::clone(&clone_iters), bg_col);
                            //let lights_direct = light_hits(&r, Arc::clone(&light_arc), Arc::clone(&hit_arc));
                            pixel = pixel + ambient_indirect;// + lights_direct;
                        }                   
                        pixel.write_col_to(curr_pixel, 0);
                    });

                    j = if j == image_width-1 { i -= 1; 0 } else { j+1};
                },
                None => break,
            }
        }        
    }

    let unwrapped_iter_vec = arc_iters.lock().unwrap();
    let total_iters: i32 = unwrapped_iter_vec.iter().sum();
    let avg_iters = total_iters / unwrapped_iter_vec.len() as i32;
    eprintln!("Average steps per ray: {}", avg_iters);


    eprintln!("Tasks finished running at {} ms", timer.ms());
    {
        let mut vec = arc_cols.lock().unwrap();
        for i in (0..vec.len()) {
            vec[i].lock().unwrap().write_color(samples as f64);
            //vec[i].write_color(samples as f64);
        }
    }

    eprintln!("Took {} ms", timer.ms());
} 