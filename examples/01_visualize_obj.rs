extern crate polyscope;

use std::f64::consts::PI;
static mut STOPPED_POSITIONS: Vec<[f32; 3]> = Vec::new();
static mut POSITIONS: Vec<[f32; 3]> = Vec::new();
static mut FRAME: i32 = 0;
const MAX_FRAME: i32 = 100;

fn update() {
    if polyscope::generate_imgui_button("Push") {
        println!("Pushed");
    }

    unsafe {
        let raw: *mut i32 = &mut FRAME;
        polyscope::generate_imgui_slider_int("frame", raw, 0, MAX_FRAME);
        FRAME += 1;
        if FRAME > MAX_FRAME {
            FRAME = 0;
        }
        for vid in 0..POSITIONS.len() {
            POSITIONS[vid][0] =
                STOPPED_POSITIONS[vid][0] * (1.0 + (FRAME as f32 / MAX_FRAME as f32));
            POSITIONS[vid][1] =
                STOPPED_POSITIONS[vid][1] * (1.0 + (FRAME as f32 / MAX_FRAME as f32));
            POSITIONS[vid][2] =
                STOPPED_POSITIONS[vid][2] * (1.0 + (FRAME as f32 / MAX_FRAME as f32));
        }
        polyscope::register_point_cloud("dynamic", &POSITIONS);
    }
}

fn main() {
    let r = 2.0;
    let num_vertices = 1000;
    let mut scalars = Vec::new();
    for vtx in 0..num_vertices {
        let cos = (2.0 * (PI as f32) * (vtx as f32 / num_vertices as f32)).cos();
        let sin = (2.0 * (PI as f32) * (vtx as f32 / num_vertices as f32)).sin();
        unsafe {
            STOPPED_POSITIONS.push([r * sin, r * cos * cos, r * cos * sin]);
            POSITIONS.push([r * sin, r * cos * cos, r * cos * sin]);
        }
        scalars.push(r * sin);
    }

    polyscope::init();
    polyscope::register_callback(update);
    unsafe {
        let ps = polyscope::register_point_cloud("stopped", &STOPPED_POSITIONS);
        polyscope::add_point_scalar_quantity(ps, "magnitude", &scalars, true);
    }
    polyscope::show();
}
