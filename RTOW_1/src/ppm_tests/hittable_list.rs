/// 6.5 - List of Hittable Objects

use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*
};

pub fn object_list_test() {
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
    
    // Objects
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(sphere::from(point3::from(0., 0., -1.), 0.5)));
    hittables.push(Box::new(sphere::from(point3::from(0., -100.5, -1.), 100.)));

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
            let pixel = ray_hits(&r, &hittables);
            pixel.write_color(1.);
        }
    }
    eprintln!("\nDone");
}

fn ray_hits(r: &ray, obj: &Vec<Box<dyn Hittable>>) ->  colorRGB {
    let mut rec = hit_record::new();
    if hit_list(obj, 0., std::f64::INFINITY, &mut rec, r) {
        return (rec.n + 1.) * 0.5
    }

    let unit_dir = r.dir.unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    colorRGB::from(1.,1.,1.)*(1.0 - t) + colorRGB::from(0.5, 0.7, 1.0) * t
}