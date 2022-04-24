pub mod motion_blur;
pub mod bvh_test;
pub mod use_textures;
pub mod use_noise;
pub mod rayon_test;

pub mod texture_map;
pub mod use_emissive;
pub mod cornell_box;

use crate::taskrunner::*;
use crate::threadpool::*;
use std::sync::mpsc;

use simple_stopwatch::Stopwatch;

use crate::objects::prelude::*;

use crate::rtow_math::prelude::*;

use crate::materials::prelude::*;
use std::sync::*;

static samples: i32 = 100;
static depth: i32 = 50;

pub fn base_cam() -> (camera, i32, i32) {
    let aspect_ratio = 16. / 9.;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.1;
    (
        camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist, 0., 1.),
        image_width,
        image_height,
    )
}

pub fn setup_direct_lights() -> Vec<light> {
    let mut lights: Vec<light> = Vec::new();
    lights.push(light::new_point(point3::from(2., 1., -2.), 10., colorRGB::from(1., 0.,0.)));
    lights.push(light::new_point(point3::from(2., 1., 2.),  10., colorRGB::from(0., 0.,1.)));
    lights.push(light::new_point(point3::from(0., 1., 2.), 10., colorRGB::from(0., 1.,0.)));
    
    lights
}

pub fn setup_objects() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.5,0.5,0.5), tex: Arc::new(Checkerboard_Tex::new()),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., Arc::clone(& material_vec[0]))));
    
    for i in (-11..11) {
        for j in (-11..11) {
            let mat_rng = rand_f64();
            let center = point3::from(i as f64 + 0.9 * rand_f64(), 0.2, j as f64 + 0.9*rand_f64());
            if(center - point3::from(4.,0.2,0.)).length() > 0.9 {
                if (mat_rng < 0.2) { // diffuse
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    material_vec.push(Arc::new(lambertian{albedo, tex: Arc::new(Solid_Color::from_colorRGB(albedo)),}));
                    let s = material_vec.len();
                    hittables.obj_list.push(Arc::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else if mat_rng < 0.8 {
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    material_vec.push(Arc::new(lambertian{albedo, tex: Arc::new(Solid_Color::from_colorRGB(albedo)),}));
                    let s = material_vec.len();
                    let mov_sph = moving_sphere::from_all(
                        center, 
                        center + point3::from(0., 0.5, 0.), 
                        0., 
                        1., 
                        0.2,
                        material_vec[s-1].clone());

                    hittables.obj_list.push(Arc::new(moving_sphere::from_all(
                        center, 
                        center + point3::from(0., 0.5, 0.), 
                        0., 
                        1., 
                        0.2,
                        material_vec[s-1].clone())));
                }
                 else if mat_rng < 0.95 { // metal
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let fuzz = rand_f64_r(0., 0.5);
                    material_vec.push(Arc::new(metal{albedo, fuzz, tex: Arc::new(Solid_Color::from_colorRGB(albedo)),}));
                    let s = material_vec.len();
                    hittables.obj_list.push(Arc::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                } else { // glass
                    let albedo = colorRGB::from(rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.), rand_f64_r(0.5, 1.));
                    let index_refr = rand_f64_r(1., 2.);
                    let alpha = rand_f64_r(0., 0.5);
                    material_vec.push(Arc::new(dielectric{albedo, alpha, index_refr, tex: Arc::new(Solid_Color::from_colorRGB(albedo)),}));
                    let s = material_vec.len();
                    hittables.obj_list.push(Arc::new(sphere::from_mat(center, 0.2, material_vec[s-1].clone())));
                }
            }
        }
    }
    material_vec.push(Arc::new(dielectric{albedo: colorRGB::from(1., 1.,1.), alpha: 0., index_refr: 1.5, tex: Arc::new(Solid_Color::from_colorRGB(colorRGB::from(1., 1.,1.))),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Arc::new(metal{albedo: colorRGB::from(0.7, 0.6, 0.5), fuzz: 0., tex: Arc::new(Solid_Color::from_colorRGB(colorRGB::from(0.7, 0.6, 0.5))),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(0.7,0.6,0.5), tex: Arc::new(Solid_Color::from_colorRGB(colorRGB::from(0.7,0.6,0.5))),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(-4., 1., 0.), 1., material_vec[material_vec.len()-1].clone())));

    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}


pub fn cam_scene2() -> (camera, i32, i32) {
    let aspect_ratio = 16. / 9.;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.;
    (
        camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist, 0., 1.),
        image_width,
        image_height,
    )
}

pub fn obj_scene2() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Checkerboard_Tex::new()),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., -10., 0.), 10., Arc::clone(& material_vec[0]))));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Checkerboard_Tex::new()),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 10., 0.), 10., Arc::clone(& material_vec[1]))));
    
    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}

pub fn obj_scene3() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Perlin_Noise::new_scaled(2.)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., Arc::clone(& material_vec[0]))));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Perlin_Noise::new_scaled(5.)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 2., 0.), 2., Arc::clone(& material_vec[1]))));
    
    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}

pub fn obj_scene4_earth() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();
    let path = String::from("earthmap.jpg");
    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(RTOW_Image::load(&path)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 0., 0.), 1., Arc::clone(&material_vec[0]))));
    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}

pub fn cam_scene5_emissive() -> (camera, i32, i32) {
    let aspect_ratio = 16. / 9.;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(26.,3.,6.);
    let lookat = point3::from(0.,2.,0.);
    let vup = vec3::from(0., 1.,0.);
    let focus_dist = (og - lookat).length();
    let aperture = 0.;
    (
        camera::from_all(og, lookat, vup, 20., aspect_ratio, aperture, focus_dist, 0., 1.),
        image_width,
        image_height,
    )
}

pub fn obj_scene5_emissive() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Solid_Color::from(0.1, 1., 0.2)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., -1000., 0.), 1000., Arc::clone(& material_vec[0]))));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(Perlin_Noise::new_scaled(5.)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 2., 0.), 2., Arc::clone(& material_vec[1]))));
    
    material_vec.push(Arc::new(Diffuse_Emissive{albedo: colorRGB::from(1.,1.,1.), tex: Arc::new(Solid_Color::from(1.,1., 1.))}));
    hittables.obj_list.push(Arc::new(xy_rect::from(3., 5., 1., 3., -4., Arc::clone(&material_vec[2]))));

    let path = String::from("earthmap.jpg");
    material_vec.push(Arc::new(Diffuse_Emissive{albedo: colorRGB::from(1.,1.,1.), tex: Arc::new(RTOW_Image::load(&path))}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 6., 0.), 1., Arc::clone(& material_vec[3]))));

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::from(1.,1.,1.), tex: Arc::new(RTOW_Image::load(&path))}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(0., 5., -3.), 1., Arc::clone(& material_vec[4]))));

    //material_vec.push(Arc::new(lambertian{albedo: colorRGB::new(), tex: Arc::new(RTOW_Image::load(&path)),}));
    hittables.obj_list.push(Arc::new(sphere::from_mat(point3::from(-1., 1., 3.), 1., Arc::clone(& material_vec[4]))));

    hittables.obj_list.push(Arc::new(xy_rect::from(3., 5., 2., 4., -2., Arc::clone(&material_vec[4]))));


    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}

pub fn cam_scene6_cornellbox() -> (camera, i32, i32) {
    let aspect_ratio = 1.;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let iw_f64 = image_width as f64;
    let ih_f64 = image_height as f64;
    let focal_length = 1.;
    
    let og = point3::from(278.,278.,-800.);
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

pub fn obj_scene6_CornellBox() -> (hittable_list, Vec<Arc<dyn Material>>) {
    let mut hittables: hittable_list = hittable_list::new();

    let mut material_vec : Vec<Arc<dyn Material>> = Vec::new();

    material_vec.push(Arc::new(lambertian{albedo: colorRGB::one(), tex: Arc::new(Solid_Color::from(0.65, 0.05, 0.05))}));
    material_vec.push(Arc::new(lambertian{albedo: colorRGB::one(), tex: Arc::new(Solid_Color::from(0.73, 0.73, 0.73))}));
    material_vec.push(Arc::new(lambertian{albedo: colorRGB::one(), tex: Arc::new(Solid_Color::from(0.12, 0.45, 0.15))}));
    material_vec.push(Arc::new(Diffuse_Emissive{albedo: colorRGB::from(1.,1.,1.), tex: Arc::new(Solid_Color::from(15., 15., 15.))}));
    hittables.obj_list.push(Arc::new(yz_rect::from(0., 555., 0., 555., 555., Arc::clone(&material_vec[2]))));
    hittables.obj_list.push(Arc::new(yz_rect::from(0., 555., 0., 555., 0., Arc::clone(&material_vec[0]))));
    hittables.obj_list.push(Arc::new(xz_rect::from(213., 343., 227., 332., 554., Arc::clone(&material_vec[3]))));
    hittables.obj_list.push(Arc::new(xz_rect::from(0., 555., 0., 555., 0., Arc::clone(&material_vec[1]))));
    hittables.obj_list.push(Arc::new(xz_rect::from(0., 555., 0., 555., 555., Arc::clone(&material_vec[1]))));
    hittables.obj_list.push(Arc::new(xy_rect::from(0., 555., 0., 555., 555., Arc::clone(&material_vec[1]))));

    hittables.obj_list.push(Arc::new(aa_box::from(point3::from(130., 0., 65.), point3::from(295., 165., 230.), Arc::clone(&material_vec[1]))));
    hittables.obj_list.push(Arc::new(aa_box::from(point3::from(265., 0., 295.), point3::from(430., 330., 460.), Arc::clone(&material_vec[1]))));

    hittables.construct_bvh(0., 1.);

    (hittables, material_vec)
}