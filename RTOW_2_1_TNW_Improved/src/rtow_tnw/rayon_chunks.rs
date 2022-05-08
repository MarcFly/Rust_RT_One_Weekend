use std::sync::mpsc;

use crate::materials::textures::*;

use simple_stopwatch::Stopwatch;

use crate::objects::prelude::*;
use crate::rtow_math::prelude::*;
use crate::materials::prelude::*;
use std::sync::*;

unsafe impl Send for ScanlineGroup {}
unsafe impl Sync for ScanlineGroup {}

struct ScanlineGroup {
    pub pixels: Box<Vec<Par_Pixel>>,
    objs: Arc<hittable_list>,
    bg_col: colorRGB,
}

impl ScanlineGroup {
    pub fn new(objs: Arc<hittable_list>, bg_col: colorRGB) -> ScanlineGroup {
        ScanlineGroup {
            pixels: Box::new(Vec::new()),
            objs,
            bg_col,
        }
    }
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

fn ray_hits(r: &ray, obj: &Arc<hittable_list>, depth_: i32, bg_col: colorRGB, last_col: colorRGB) ->  (ray, colorRGB, colorRGB, bool) {
    if(depth_ < 1) {return (ray::new(), colorRGB::new(), colorRGB::one(), true)}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    let mut attenuation = colorRGB::new();

    if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r) {
        return (ray::new(), colorRGB::new(), bg_col, true);
    }

    let mut scattered = ray::new();
    let emitted = rec.mat.emitted(rec.uv.v[0], rec.uv.v[1], &rec.p);

    if !rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered) {
        return (scattered, emitted, colorRGB::new(), true);
    }
    
    (
        scattered,
        emitted,
        attenuation,
        false
    )
}

#[derive(Copy, Clone)]
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
    let arc_hit = Arc::new(hittables);
        
    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    let mut scanlines: Box<Vec<ScanlineGroup>> = Box::new(Vec::new());

    let num_groups = image_height;
    {
        let sl_per_group = 1 + image_height / num_groups;
        for i in 0..num_groups {
            scanlines.push(ScanlineGroup::new(Arc::clone(&arc_hit), bg_col));
        }

        for i in (0..image_height).rev() {
            let curr_group = (i / sl_per_group) as usize;
            for j in 0..image_width {
                scanlines[curr_group].pixels.push(Par_Pixel{ color: colorRGB::new(), i, j});
            }    
        }
    }
    eprintln!("Finished creating individual ArcMutexColorRGB at {} ms", timer.ms());

    let mut arc_iters: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    //eprintln!("i = {} / j = {}", pixel.i, pixel.j);
    if let Some(usage) = memory_stats() {
            eprintln!("Iterative PreLoop Physical Mem: {}", usage.physical_mem / 1024 / 1024);
            eprintln!("Iterative PreLoop Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };

    eprintln!();

    {
        let par_iter = scanlines.into_par_iter().map(|mut group| {

            for i in 0..group.pixels.len() {
                let mut pixel = &mut group.pixels[i];

                let mut ambient_indirect = colorRGB::new();
                let mut attenuation_bounces = colorRGB::one();
                
                let mut early_out = false;
                let mut step_col = colorRGB::new();
                let mut step_emit = colorRGB::new();
                
                //let mut out_pixel = Par_Pixel{color: colorRGB::new(), i: pixel.i, j: pixel.j};
                for s in (0..samples) {
                    let u = (pixel.j as f64 + rand_f64()) / (iw_f64 - 1.);
                    let v = (pixel.i as f64 + rand_f64()) / (ih_f64 - 1.);
                    let mut r = cam.focus_time_ray(u, v);
                    
                    ambient_indirect = colorRGB::new();
                    attenuation_bounces = colorRGB::one();

                    for it_depth in (0..depth).rev()
                    {
                        (r, step_emit, step_col, early_out) = ray_hits(&r, &group.objs, depth, bg_col, ambient_indirect);
                        if !step_col.near_zero() {
                            attenuation_bounces = attenuation_bounces * step_col;
                        }
                        ambient_indirect = ambient_indirect + (step_emit * attenuation_bounces);
                    
                        if(early_out) {break};

                    }
                
                    pixel.color = pixel.color + ambient_indirect;
                } 
            }
        

        group
    });
        
        scanlines = Box::new(Vec::from_par_iter(par_iter));
    }

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostLoop Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostLoop Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
    eprintln!();

    eprintln!("Tasks finished running at {} ms", timer.ms());
    {
        //let mut vec = arc_cols.lock().unwrap();
        let _len = scanlines.len();
        for i in (0.._len).rev() {
            for j in (0..scanlines[i].pixels.len()) {
                scanlines[i].pixels[j].color.write_color(samples as f64);
            }
        }
    }
//
    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostWriteFile Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostWriteFile Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
    eprintln!();

    eprintln!("Took {} ms", timer.ms());
}