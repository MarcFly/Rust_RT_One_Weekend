#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

mod rtow_math;
mod materials;

mod unoptimized;
mod optimized;

mod threadpool;

use simple_stopwatch::Stopwatch;
use unoptimized::Unoptimized;

fn main() {
    let mut timer = Stopwatch::start_new();

    Unoptimized();



    eprintln!("Took {} ms", timer.ms());
}