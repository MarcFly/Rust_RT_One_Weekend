#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_improvements;
use crate::rtow_improvements::light_shadow_rays::*;

mod rtow_math;
mod materials;
mod objects;
mod threadpool;
mod taskrunner;
mod rtow_tnw;

use simple_stopwatch::Stopwatch;

fn main() {
    // rtow_improvements();
    rtow_tnw_fns();

}

fn rtow_improvements() {
    shadow_rays();
}


use crate::rtow_tnw::*;
fn rtow_tnw_fns() {
    //motion_blur::render();
    //bvh_test::render();
    //use_textures::render();
    //use_noise::render();
    texture_map::render();
    //rayon_test::render();
    //ayon_test::render_no_rayon();
}