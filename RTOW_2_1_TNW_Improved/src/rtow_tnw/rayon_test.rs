use std::sync::mpsc;

use crate::materials::textures::*;

use simple_stopwatch::Stopwatch;

use crate::objects::prelude::*;
use crate::rtow_math::prelude::*;
use crate::materials::prelude::*;
use std::sync::*;

struct next_step {
    r: ray,

}

// Per step we have Current Emission + Current Attenuation * ColorChain
// In recursive it just works
// In iterative we don't have ColorChain, we are going front->back not back->front for result
// If we encounter 0., we lose all chain
// If we convert loss to 1, we just don't care about background and all becomes fucked up

// To each step, we have to Add emission post Attenuation mul
// So stat with default emission and attenuation separate
// We get current step
// En, An
// (En+1 + An+1) * An + En = An+1
// (En+2  + An+2) * An+1 + En+1 = An+2

fn ray_hits(r: &ray, obj: &Arc<hittable_list>, depth_: i32, debug_iter_vec: &Arc<Mutex<Vec<i32>>>, bg_col: colorRGB, last_col: colorRGB) ->  (ray, colorRGB, colorRGB, bool) {
    if(depth_ < 1) {return (ray::new(), colorRGB::new(), colorRGB::one(), true)}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    let mut attenuation = colorRGB::new();

    if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r) {
        debug_iter_vec.lock().unwrap().push(rec.iters);
        return (ray::new(), colorRGB::new(), bg_col, true);
    }
    if rec.uv.v[0].is_nan() || rec.uv.v[1].is_nan() {
        let mut test = false;
        test = true;
    }
    let mut scattered = ray::new();
    let emitted = rec.mat.emitted(rec.uv.v[0], rec.uv.v[1], &rec.p);

    if !rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered) {
        debug_iter_vec.lock().unwrap().push(rec.iters);
        return (scattered, emitted, colorRGB::one(), true);
    }

    debug_iter_vec.lock().unwrap().push(rec.iters);
    
    (
        scattered,
        emitted,
        attenuation,
        false
    )
}

enum Pixel {
    RGB(usize, colorRGB),
}

use crate::rtow_tnw::*;

use rayon::prelude::*;
use memory_stats::memory_stats;

pub fn render() {
    let mut timer = Stopwatch::start_new();

    let (cam, image_width, image_height) = cam_final_scene();
    let (iw_f64, ih_f64) = (image_width as f64, image_height as f64);
    let bg_col = colorRGB::new();

    // SETUP Objects and materials 
    
    let (mut hittables, mut material_vec) = obj_final_scene();

    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    let mut cols: Box<Vec<Par_Pixel>> = Box::new(Vec::new());
    {
        cols.reserve((image_height * image_width) as usize);
        for i in (0..image_height).rev() {
            for j in 0..image_width {
                cols.push(Par_Pixel{
                    color:colorRGB::new(),
                    i, 
                    j,
                });
            }
        }
        //cols.resize(image_width * image_height as usize, (colorRGB::new());
    }
    eprintln!("Finished creating individual ArcMutexColorRGB at {} ms", timer.ms());

    let arc_hit = Arc::new(hittables);
    let mut arc_iters: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    //eprintln!("i = {} / j = {}", pixel.i, pixel.j);
    if let Some(usage) = memory_stats() {
            eprintln!("Iterative PreLoop Physical Mem: {}", usage.physical_mem / 1024 / 1024);
            eprintln!("Iterative PreLoop Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
        };
    eprintln!();

    {
        let par_iter = cols.into_par_iter().map(|mut pixel| {

            let pxl_iters = Arc::clone(&arc_iters);
            let pxl_hit = Arc::clone(&arc_hit);
            
            let mut it_depth = depth;
            let mut ambient_indirect = colorRGB::one();
            let mut step_col = colorRGB::new();
            let mut early_out = false;
            let mut step_emit = colorRGB::new();

            for s in (0..samples) {
                let u = (pixel.j as f64 + rand_f64()) / (iw_f64 - 1.);
                let v = (pixel.i as f64 + rand_f64()) / (ih_f64 - 1.);
                let mut r = cam.focus_time_ray(u, v);

                for it_depth in (0..depth).rev()
                {
                    (r, step_emit, step_col, early_out) = ray_hits(&r, &pxl_hit, depth, &pxl_iters, bg_col, ambient_indirect);
                    
                    ambient_indirect = step_emit * ambient_indirect + ambient_indirect * step_col;

                    if(early_out) {break};
                    
                }

                pixel.color = pixel.color + ambient_indirect;
            }   
            if pixel.i as f64 % 50. < 0.00001 && pixel.j as f64 % 50. < 0.00001  {
                
            };
                          
            pixel

        });
        cols = Box::new(Vec::from_par_iter(par_iter));
    }

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostLoop Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostLoop Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
    eprintln!();

    let unwrapped_iter_vec = arc_iters.lock().unwrap();
    let total_iters: i32 = unwrapped_iter_vec.iter().sum();
    let avg_iters = total_iters / unwrapped_iter_vec.len() as i32;
    eprintln!("Average steps per ray: {}", avg_iters);


    eprintln!("Tasks finished running at {} ms", timer.ms());
    {
        //let mut vec = arc_cols.lock().unwrap();
        let _len = cols.len();
        for i in (0.._len) {
            //vec[i].lock().unwrap().write_color(samples as f64);
            //vec[i].write_color(samples as f64);
            cols[i].color.write_color(samples as f64);
        }
    }

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostWriteFile Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostWriteFile Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
    eprintln!();

    eprintln!("Took {} ms", timer.ms());
}