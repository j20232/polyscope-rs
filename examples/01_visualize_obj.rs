extern crate nalgebra as na;
extern crate polyscope as ps;

use std::f64::consts::PI;
static mut STOPPED_POSITIONS: Vec<[f32; 3]> = Vec::new();
static mut POSITIONS: Vec<[f32; 3]> = Vec::new();
static mut FRAME: i32 = 0;
const MAX_FRAME: i32 = 100;

fn update() {
    if ps::imgui::button("Push") {
        println!("Pushed");
    }

    unsafe {
        let raw: *mut i32 = &mut FRAME;
        ps::imgui::slider_int("frame", raw, 0, MAX_FRAME);
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
        <ps::point_cloud as ps::pc_vec3<Vec<[f32; 3]>>>::register_point_cloud(
            "dynamic", &POSITIONS,
        );
    }
}

fn main() {
    let r = 2.0;
    let num_vertices = 1000;
    let mut scalars = Vec::new();
    let mut doubles = Vec::new();
    let mut colors = Vec::new();
    for vtx in 0..num_vertices {
        let cos = (2.0 * (PI as f32) * (vtx as f32 / num_vertices as f32)).cos();
        let sin = (2.0 * (PI as f32) * (vtx as f32 / num_vertices as f32)).sin();
        unsafe {
            STOPPED_POSITIONS.push([r * sin, r * cos * cos, r * cos * sin]);
            POSITIONS.push([r * sin, r * cos * cos, r * cos * sin]);
        }
        scalars.push(r * sin);
        doubles.push((r * cos) as f64);
        colors.push([cos * cos, cos * sin, sin]);
    }

    ps::init();
    ps::register_callback(update);

    // nalgebra
    let mut dm = na::DMatrix::<f64>::zeros(4, 3);
    for i in 0..dm.nrows() {
        dm[(i, 0)] = i as f64;
        dm[(i, 1)] = i as f64 * 0.1;
        dm[(i, 2)] = i as f64 * 0.2;
    }

    <ps::point_cloud as ps::pc_vec3<na::DMatrix<f64>>>::register_point_cloud(
        "nalgebra DMatrix",
        &dm,
    );

    // Vec<[T; 3]>
    unsafe {
        let ps = <ps::point_cloud as ps::pc_vec3<Vec<[f32; 3]>>>::register_point_cloud(
            "stopped",
            &STOPPED_POSITIONS,
        );
        <ps::point_cloud as ps::pc_scalar<Vec<f32>>>::add_scalar_quantity(
            ps, "mag", &scalars, true,
        );
        <ps::point_cloud as ps::pc_scalar<Vec<f64>>>::add_scalar_quantity(
            ps, "m_double", &doubles, true,
        );
        <ps::point_cloud as ps::pc_vec3<Vec<[f32; 3]>>>::add_color_quantity(
            ps, "colors", &colors, true,
        );
        <ps::point_cloud as ps::pc_vec3<Vec<[f32; 3]>>>::add_vector_quantity(
            ps, "vecs", &colors, true,
        );
    }
    ps::show();
}
