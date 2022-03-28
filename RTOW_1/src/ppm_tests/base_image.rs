/// 1-3 - Base Image and utilities 

pub fn output_ppm() {
    // Image
    let w: i32 = 256;
    let h: i32 = 256;
    let float_w: f64 = 256.;
    let float_h: f64 = 256.;

    // Render
    println!("P3\n{} {}\n255\n", w,h);
    
    for i in (0..(h-1)).rev() {
        let float_i: f64 = (i).into();
        eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..w) {
            let float_j: f64 = (j).into();
            let r: f64 = float_j / (float_w-1.);
            let g: f64 = float_i / (float_h-1.);
            let b: f64 = 0.25;
            
            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            print!("{} {} {}\n",ir,ig,ib);
        }
    }
    eprintln!("\nDone");
}

use crate::rtow_math::vec3::*;

pub fn output_ppm_vec3() {
    // Image
    let w: i32 = 256;
    let h: i32 = 256;
    let float_w: f64 = 256.;
    let float_h: f64 = 256.;

    // Render
    println!("P3\n{} {}\n255\n", w,h);
    
    for i in (0..(h)).rev() {
        let float_i: f64 = (i).into();
        eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..w) {
            let float_j: f64 = (j).into();
            let pixel_color = vec3::from(
                float_j / (float_w-1.),
                float_i / (float_h-1.),
                0.25
            );
            
            pixel_color.write_color(1.);
        }
    }
    eprintln!("\nDone");
}