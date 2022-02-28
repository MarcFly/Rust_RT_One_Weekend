pub fn output_ppm() {
    // Image
    let w: i32 = 256;
    let h: i32 = 256;
    let float_w: f64 = 256.;
    let float_h: f64 = 256.;

    // Render
    println!("P3\n{}' '{}\n255\n", w,h);
    
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
    println!("P3\n{}' '{}\n255\n", w,h);
    
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
            
            pixel_color.write_color();
        }
    }
    eprintln!("\nDone");
}

use crate::rtow_math::ray::*;

fn ray_color(r: &ray) -> colorRGB {
    let unit_dir = r.dir.unit_vec();
    let t = (unit_dir.y() + 1.) * 0.5;
    let inv_t = 1.-t;
    let col2 = colorRGB::from(0.5,0.7,1.0) * t;
    colorRGB::from(1.,1.,1.) * inv_t + col2
}

pub fn output_ppm_cam() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let vp_h = 2.0;
    let vp_w = aspect_ratio * vp_h;
    let focal_len = 1.0;

    let origin = point3::new();
    let horizontal = vec3::from(vp_w, 0.,0.);
    let vertical = vec3::from(0., vp_h, 0.);
    let lower_left = origin - horizontal / 2. - vertical / 2. - vec3::from(0.,0., focal_len);

    // Render

    // Render
    println!("P3\n{}' '{}\n255\n", image_width, image_height);
    
    for i in (0..(image_height)).rev() {
        let float_i: f64 = (i).into();
        eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..image_width) {
            let float_j: f64 = (j).into();
            let u = float_j / (image_width as f64 - 1.);
            let v = float_i / (image_height as f64 - 1.);
            let dir = lower_left + horizontal * u + vertical * v - origin;
            let r = ray::from(&origin, &dir);
            let pixel = ray_color(&r);
            pixel.write_color();
        }
    }
    eprintln!("\nDone");
}