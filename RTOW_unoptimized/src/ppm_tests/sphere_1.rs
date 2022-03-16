/// 5 - Simple Sphere

use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
};

use crate::rtow_math::sphere::*;

fn ray_color_sphere(r: &ray) -> colorRGB {
    let sphere = point3::from(0.,0.,-1.);
    if(hit_sphere(&sphere, 0.5, r) > 0.) {
        return colorRGB::from(1.,0.,0.);
    }

    let unit_dir = r.dir.unit_vec();
    let t = (unit_dir.y() + 1.) * 0.5;
    let inv_t = 1.-t;
    let col2 = colorRGB::from(0.5,0.7,1.0) * t;
    colorRGB::from(1.,1.,1.) * inv_t + col2
}

pub fn output_ppm_sphere() {
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

    // The lower left of the Viewport depends first on the intended size
    // We assume origin at an arbitrary point, generic viewport would use -pixel_max, pixel_max as limits
    // But we transform that according to the focal lenght, which will change how far things are basically
    let lower_left = origin - horizontal / 2. - vertical / 2. - vec3::from(0.,0., focal_len);

    // Render

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    // Throw a ray at every pixel
    for i in (0..(image_height)).rev() {
        let float_i: f64 = (i).into();
        eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..image_width) {
            let float_j: f64 = (j).into();

            // Transform pixel coordinates to uv, aka form full size into 0. to 1. range
            let u = float_j / (image_width as f64 - 1.);
            let v = float_i / (image_height as f64 - 1.);
            let dir = lower_left + horizontal * u + vertical * v - origin;
            let r = ray::from(&origin, &dir);
            let pixel = ray_color_sphere(&r);
            pixel.write_color(1.);
        }
    }
    eprintln!("\nDone");
}