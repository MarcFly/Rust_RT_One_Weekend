use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Rust_RT_One_Weekend::rtow_tnw::*;
use Rust_RT_One_Weekend::materials::prelude::*;
use Rust_RT_One_Weekend::objects::prelude::*;
use Rust_RT_One_Weekend::rtow_math::prelude::*;
use std::sync::*;

use memory_stats::memory_stats;

fn ray_hits_iterative(r: &ray, obj: &Arc<hittable_list>, depth_: i32, bg_col: colorRGB, last_col: colorRGB) ->  (ray, colorRGB, colorRGB, bool) {
    if(depth_ < 1) {return (ray::new(), colorRGB::new(), colorRGB::one(), true)}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    let mut attenuation = colorRGB::new();

    if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r) {
        return (ray::new(), colorRGB::new(), bg_col, true);
    }
    if rec.uv.v[0].is_nan() || rec.uv.v[1].is_nan() {
        let mut test = false;
        test = true;
    }
    let mut scattered = ray::new();
    let emitted = rec.mat.emitted(rec.uv.v[0], rec.uv.v[1], &rec.p);

    if !rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered) {
        return (scattered, emitted, colorRGB::one(), true);
    }
    
    (
        scattered,
        emitted,
        attenuation,
        false
    )
}

fn ray_hits_recursive(r: &ray, obj: Arc<hittable_list>, depth_: i32, bg_col: colorRGB) ->  colorRGB {
    if(depth_ < 1) {return colorRGB::new()}

    let next_depth = depth_ -1;
    
    let mut rec = hit_record::new();
    let mut attenuation = colorRGB::new();

    if !obj.hit_bvh(0.0001, std::f64::INFINITY, &mut rec, r) {
        return bg_col;
    }
    if rec.uv.v[0].is_nan() || rec.uv.v[1].is_nan() {
        let mut test = false;
        test = true;
    }
    let mut scattered = ray::new();
    let emitted = rec.mat.emitted(rec.uv.v[0], rec.uv.v[1], &rec.p);

    if !rec.mat.scatter_tex(r, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    emitted + ray_hits_recursive(&scattered, obj, next_depth, bg_col) * attenuation
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Prepare data
    let (cam, image_width, image_height) = cam_final_scene();
    let (iw_f64, ih_f64) = (image_width as f64, image_height as f64);
    let bg_col = colorRGB::new();
    let samples = 50;
    let depth = 10;
    // SETUP Objects and materials 
    
    let (mut hittables, mut material_vec) = obj_final_scene();

    println!("P3\n{} {}\n255\n", image_width, image_height);
    
   

    // Parallel Pixels for rayon version
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
    }
    
    // Parallel RGB for non-rayon version
    let mut arc_cols: Arc<Mutex<Box<Vec<Arc<Mutex<colorRGB>>>>>> = Arc::new(Mutex::new(Box::new(Vec::new())));
    {
        let mut vec = arc_cols.lock().unwrap();
        for i in 0..image_width * image_height {
            vec.push(Arc::new(Mutex::new(colorRGB::new())));
        }
        //vec.resize(image_width * image_height as usize, colorRGB::new());
    }

    let arc_hit = Arc::new(hittables);
    let arc_iters: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    if let Some(usage) = memory_stats() {
        eprintln!("Recursive PreTest Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Recursive PreTest Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };

    c.bench_function("Recursive", |b| b.iter(|| {
        let mut j = rand_f64_r(0., image_width as f64).floor() as usize;
        let mut i = rand_f64_r(0., image_height as f64).floor() as usize;
        //eprintln!("i = {} / j {}", i , j);
        let curr_pixel = Arc::clone(&arc_cols.lock().unwrap()[(j + i * (image_width as usize))]);

        let mut pixel = colorRGB::new();
        for s in (0..samples) {
            //let x_sum = (s as f64).sqrt()
            let u = (j as f64 + rand_f64()) / (iw_f64 - 1.);
            let v = (i as f64 + rand_f64()) / (ih_f64 - 1.);
            let r = cam.focus_time_ray(u, v);
            let ambient_indirect = ray_hits_recursive(&r, Arc::clone(&arc_hit), depth, bg_col);
            
            pixel = pixel + ambient_indirect;// + lights_direct;
        }                   
        pixel.write_col_to(curr_pixel, 0);
    }));
    if let Some(usage) = memory_stats() {
        eprintln!("Recursive PostTest Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Recursive PostTest Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };

    eprintln!("\nSleep 10 secs to reset mem\n");
    std::thread::sleep(std::time::Duration::from_secs(10));

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PreTest Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PreTest Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };

    c.bench_function("Iterative", |b| b.iter(|| {
        let mut pixel = cols[rand_f64_r(0., cols.len() as f64).floor() as usize].clone();
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
                    (r, step_emit, step_col, early_out) = ray_hits_iterative(&r, &pxl_hit, depth, bg_col, ambient_indirect);
                    
                    ambient_indirect = step_emit * ambient_indirect + ambient_indirect * step_col;

                    if(early_out) {break};
                    
                }

                pixel.color = pixel.color + ambient_indirect;
            }
    }));

    if let Some(usage) = memory_stats() {
        eprintln!("Iterative PostTest Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Iterative PostTest Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    };
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);