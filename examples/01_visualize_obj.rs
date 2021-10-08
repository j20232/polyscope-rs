extern crate polyscope;
extern crate tobj;

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
    let obj_file = "./assets/omesis/unchan_pink.obj";
    let (models, _materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");
    let mesh = &models[0].mesh;
    let flat_pos = &mesh.positions;
    let print = update;

    let mut scalars = Vec::new();
    for vtx in 0..mesh.positions.len() / 3 {
        unsafe {
            STOPPED_POSITIONS.push([
                flat_pos[3 * vtx],
                flat_pos[3 * vtx + 1],
                flat_pos[3 * vtx + 2],
            ]);
            POSITIONS.push([
                flat_pos[3 * vtx],
                flat_pos[3 * vtx + 1],
                flat_pos[3 * vtx + 2],
            ]);
        }
        scalars.push(flat_pos[3 * vtx] * flat_pos[3 * vtx + 1] * flat_pos[3 * vtx + 2]);
    }

    polyscope::init();
    polyscope::register_callback(print);
    unsafe {
        let ps = polyscope::register_point_cloud("stopped", &STOPPED_POSITIONS);
        polyscope::add_point_scalar_quantity(ps, "magnitude", &scalars, true);
    }
    polyscope::show();
}
