use std::sync::mpsc;

use crate::materials::textures::*;

use simple_stopwatch::Stopwatch;

use crate::objects::prelude::*;
use crate::rtow_math::prelude::*;
use crate::materials::prelude::*;
use std::sync::*;

unsafe impl Send for TileGroup {}
unsafe impl Sync for TileGroup {}

struct TileGroup {
    pub pixels: Box<Vec<Arc<Mutex<Par_Pixel>>>>,
    objs: Arc<hittable_list>,
    bg_col: colorRGB,
}

impl TileGroup {
    pub fn new(objs: Arc<hittable_list>, bg_col: colorRGB) -> TileGroup {
        TileGroup {
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
    
    let mut image: Box<Vec<Arc<Mutex<Par_Pixel>>>> = Box::new(Vec::new());
    {
        image.reserve((image_width * image_height) as usize);
        for i in (0..(image_height as usize)).rev() {
            for j in 0..image_width {
                image.push(Arc::new(Mutex::new(Par_Pixel{ color: colorRGB::new(), i: i as i32, j: j as i32 })))
            }            
        }
    }

    let mut tiles: Box<Vec<TileGroup>> = Box::new(Vec::new());

    let pixels_per_group = 200;
    let num_groups = ((image_height * image_width) / pixels_per_group) + image_width / pixels_per_group;
    {
        let group_width = (pixels_per_group as f64).sqrt() as i32;
        let group_height = (pixels_per_group / group_width);

        for i in 0..num_groups {
            tiles.push(TileGroup::new(Arc::clone(&arc_hit), bg_col));
        }

        let groups_per_width = image_width / group_width ;
        let groups_per_height = image_height / group_height;
        for t in 0..tiles.len() {
            let t_i32 = t as i32;
            
            
            let row = ((t_i32 * group_width) as f64 / image_width as f64)  as i32;
            // +1 at groups_per_width to reset t to 0, els it would be 1+ everytime)
            // Meaning we are advancing columns, not resetting column to 0
            let j_start = (t_i32 - row * (groups_per_width + 1)) * group_width;
            let j_end = j_start + group_width;

            let i_start = row * (group_height);
            let i_end   = i_start + group_width + 1;

            for i in (i_start..i_end).rev() {
                if i > image_height {
                    let s = false;
                    continue
                };
                for j in (j_start..j_end) {
                    if j > (image_width-1) {
                        let s = false;
                        continue
                    };
                    let index = (j  + i * image_width) as usize;
                    tiles[t].pixels.push(Arc::clone(&image[index]));
                }
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
        let par_iter = tiles.into_par_iter().map(|mut group| {

            for i in 0..group.pixels.len() {
                let mut guard_pxl = group.pixels[i].lock().unwrap();
                let mut pixel = Par_Pixel{color: colorRGB::new(), i: guard_pxl.i, j: guard_pxl.j}; //&mut group.pixels[i];

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

                guard_pxl.set_col(pixel.color); 
            }
        

        group
    });
        
        tiles = Box::new(Vec::from_par_iter(par_iter));
    }

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostLoop Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostLoop Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
    eprintln!();

    eprintln!("Tasks finished running at {} ms", timer.ms());
    {
        //let mut vec = arc_cols.lock().unwrap();
        let _len = tiles.len();
        for i in 0..image.len() {
            image[i].lock().unwrap().color.write_color(samples as f64);
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