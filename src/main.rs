#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_math;
use crate::rtow_math::{
    vec3::*, 
    sphere::*, 
    ray::*,
    hit::*
};

use simple_stopwatch::Stopwatch;

fn main() {
    let mut timer = Stopwatch::start_new();
    Chapters1_7();
    Chapters8_X();



    eprintln!("Took {} ms", timer.ms());
}

mod ppm_tests;
use crate::ppm_tests::{
    antialiasing::*,
    base_image::*,
    camera_rays::*,
    hittable_list::*,
    sphere_1::*,
    sphere_normals::*,
};

fn Chapters1_7(){
    //output_ppm();
    //output_ppm_vec3();
    //output_ppm_cam();
    //output_ppm_sphere();
    //output_ppm_s_normals();
    //object_list_test();
    //output_multisample();
}

mod ppm_materials;
use crate::ppm_materials::diffuse::*;
use crate::ppm_materials::materials_1::*;
use crate::ppm_materials::materials_refract::*;
mod materials;

fn Chapters8_X() {
    //output_diffuse();
    //output_materials();
    output_materials_refract();
}