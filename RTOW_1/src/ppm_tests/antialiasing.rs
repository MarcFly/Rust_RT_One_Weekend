use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*,
    camera::*,
    rng::*,
};

fn ray_hits(r: &ray, obj: &Vec<Box<dyn Hittable>>) ->  colorRGB {
    let mut rec = hit_record::new();
    if hit_list(obj, 0., std::f64::INFINITY, &mut rec, r) {
        return (rec.n + 1.) * 0.5
    }

    let unit_dir = r.dir.unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    colorRGB::from(1.,1.,1.)*(1.0 - t) + colorRGB::from(0.5, 0.7, 1.0) * t
}

pub fn output_multisample() {
    let samples = 100;

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let focal_length = 1.;
    
    let cam = camera::from(aspect_ratio, 2. * aspect_ratio as f64, focal_length);
    //let cam = camera::new();

    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(sphere::from(point3::from(0., 0., -1.), 0.5)));
    hittables.push(Box::new(sphere::from(point3::from(0., -100.5, -10.), 100.)));
    
    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    // Throw a ray at every pixel
    for i in (0..(image_height)).rev() {
        let float_i: f64 = (i).into();
        //eprintln!("\rScanlines Remaining: {}\r", i);
        for j in (0..image_width) {
            let float_j: f64 = (j).into();
            let mut pixel = colorRGB::new();
            for s in (0..samples) {
                //let x_sum = (s as f64).sqrt()
                let u = (float_j + rand_f64()) / (image_width as f64 - 1.);
                let v = (float_i + rand_f64()) / (image_height as f64 - 1.);
                let r = cam.ray(u, v);
                pixel = pixel + ray_hits(&r, &hittables);
            }
            pixel.write_color(samples as f64);
        }
    }
    eprintln!("\nDone with {} samples per pixel", samples);

}