pub mod rayon_test;
pub mod rayon_chunks;
pub mod rayon_tiles;

pub mod final_scene_render;
use std::sync::mpsc;

use simple_stopwatch::Stopwatch;

use crate::objects::prelude::*;

use crate::rtow_math::prelude::*;

use crate::materials::prelude::*;
use std::sync::*;

static samples: i32 = 50;
static depth: i32 = 20;

pub fn cam_final_scene() -> (camera, i32, i32) {
    let aspect_ratio = 1.;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(478.,278.,-600.);
    let lookat = point3::from(278.,278.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.;
    (
        camera::from_all(og, lookat, vup, 40., aspect_ratio, aperture, focus_dist, 0., 1.),
        image_width,
        image_height,
    )
}

pub fn obj_final_scene() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();
    
    // Ground - Different Height boxes
    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.48, 0.83, 0.53), tex: Arc::new(Solid_Color::from_colorRGB(colorRGB::from(0.48, 0.83, 0.53)))}));
    let mut ground_boxes = hittable_list::new();
    for i in 0..20 {
        let f_i = i as f64;
        for j in 0..20 {
            let f_j = j as f64;

            let w = 100.;
            let x0 = -1000. + f_i * w;
            let z0 = -1000. + f_j *w;
            let y0 = 0.;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = rand_f64_r(1., 101.);
            
            ground_boxes.obj_list.push(Arc::new(aa_box::from(point3::from(x0, y0, z0), point3::from(x1, y1, z1), Arc::clone(&material_vec[0]))));
        }
    }
    ground_boxes.construct_bvh(0.,1.);
    hittables.obj_list.push(Arc::new(ground_boxes));

    // Emitters
    material_vec.push(Arc::new(Diffuse_Emissive{albedo: colorRGB::one() * 7., tex: Arc::new(Solid_Color::from_colorRGB(colorRGB::one()))}));
    hittables.obj_list.push(Arc::new(xz_rect::from(123., 423., 147., 412., 553., Arc::clone(&material_vec[1]))));

    // Base Spheres
    let c1 = point3::from(400., 400., 200.);
    let c2 = c1 + vec3::from(30.,0.,0.);

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::one(), tex: Arc::new( Solid_Color::from( 0.7, 0.3, 0.1))}));
    let mov_sphere1 = moving_sphere::from_all(c1, c2, 0., 1., 50., Arc::clone(&material_vec[2]));
    hittables.obj_list.push(Arc::new(mov_sphere1));

    material_vec.push(Arc::new( dielectric::from(0., 1.5, Arc::new(Solid_Color::from_colorRGB(colorRGB::one())))));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(260., 150., 45.), 50., Arc::clone(&material_vec[3]))));
    
    material_vec.push(Arc::new(metal::new(1., Arc::new(Solid_Color::from(0.8, 0.8, 0.8)))));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 150., 145.), 50., Arc::clone(&material_vec[4]))));

    // Sphere with Fog inside
    material_vec.push(Arc::new(dielectric::from(0., 1.5, Arc::new(Solid_Color::from_colorRGB(colorRGB::one())))));
    let boundary = sphere::from_mat(point3::from(360., 150., 145.), 70., Arc::clone(&material_vec[5]));
    let boundary2 = boundary.clone();
    hittables.obj_list.push(Arc::new(boundary));

    hittables.obj_list.push(Arc::new(constant_medium::new(Box::new(boundary2), 0.2, Arc::new(Solid_Color::from(0.2, 0.4, 0.9)))));

    // General Fog
    material_vec.push(Arc::new(dielectric::from(0., 1.5, Arc::new(Solid_Color::from_colorRGB(colorRGB::one())))));
    let boundary3 = sphere::from_mat(point3::new(), 5000., Arc::clone(&material_vec[6]));
    hittables.obj_list.push(Arc::new(constant_medium::new(Box::new(boundary3), 0.0001, Arc::new(Solid_Color::from_colorRGB(colorRGB::one())))));

    // Earth Sphere
    let path = String::from("earthmap.jpg");
    material_vec.push(Arc::new(lambertian::new(colorRGB::one(), Arc::new(RTOW_Image::load(&path)))));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(400., 200., 400.), 100., Arc::clone(&material_vec[7]))));

    // Noise Sphere
    material_vec.push(Arc::new( lambertian::new(colorRGB::new(), Arc::new(Perlin_Noise::new_scaled(0.1)))));
    hittables.obj_list.push(Arc::new( sphere::from_mat(point3::from(220., 280., 300.), 80., Arc::clone(&material_vec[8]))));

    // Spheres in a Box
    material_vec.push(Arc::new(lambertian::new(colorRGB::one(), Arc::new(Solid_Color::from(0.73, 0.73, 0.73)))));
    let ns = 1000;
    let mut sphere_in_box = hittable_list::new();
    for i in 0..1000 {
        sphere_in_box.obj_list.push(Arc::new(sphere::from_mat(random_in_unit_cube() * 165., 10., Arc::clone(&material_vec[9]))));
    }
    sphere_in_box.construct_bvh(0.,1.);

    let translated_spheres = 
    translated::new(Box::new(
        rotated::new( Box::new(
            sphere_in_box),
            vec3::from(0., 15., 0.))),
    vec3::from(-100., 270., 395.));

    hittables.obj_list.push(Arc::new(translated_spheres));
        
    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}