#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

pub mod taskrunner;
pub mod rtow_math;
pub mod materials;
pub mod objects;

pub mod rtow_tnw;

use simple_stopwatch::Stopwatch;

use tracing::subscriber;
use tracing_tracy::TracyLayer;
use tracing::{debug, event, info, info_span, span, Level};
use tracing_subscriber::layer::SubscriberExt;


fn main() {

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::new()),
    ).expect("set up the subscriber");

    // rtow_improvements();
    rtow_tnw_fns();

}


use crate::rtow_tnw::*;

use memory_stats::memory_stats;

fn rtow_tnw_fns() {
    //motion_blur::render();
    //bvh_test::render();
    //use_textures::render();
    //use_noise::render();
    //texture_map::render();
    
    //use_emissive::render();
    //cornell_box::render();
    //use_volumes::render();
    let app_span = span!(Level::TRACE, "Span1");
    let app_entry = app_span.enter();

    if let(Some(usage)) = memory_stats() {
        eprintln!("Before All Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("Before All Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
        
        //rayon_test::render();
        //rayon_chunks::render();
        event!(Level::TRACE, "Entering Render");
        rayon_tiles::render();
        eprintln!("After Iterative Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("After Iterative Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
        
        //final_scene_render::render();
        
        eprintln!("After Recursive Physical Mem: {}", usage.physical_mem / 1024 / 1024);
        eprintln!("After Recursive Virtual Mem: {}", usage.virtual_mem / 1024 / 1024);
    }
}