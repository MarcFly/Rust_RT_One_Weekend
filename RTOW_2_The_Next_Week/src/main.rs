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

use simple_stopwatch::Stopwatch;

fn main() {
    // RTOW Improvements
    shadow_rays();
}