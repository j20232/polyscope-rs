extern crate polyscope;
extern crate tobj;

fn main() {
    let obj_file = "./assets/omesis/unchan_pink.obj";
    let (models, _materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");
    let mesh = &models[0].mesh;
    let flat_pos = &mesh.positions;

    let mut positions = Vec::new();
    let mut scalars = Vec::new();
    for vtx in 0..mesh.positions.len() / 3 {
        positions.push([
            flat_pos[3 * vtx],
            flat_pos[3 * vtx + 1],
            flat_pos[3 * vtx + 2],
        ]);
        scalars.push(flat_pos[3 * vtx] * flat_pos[3 * vtx + 1] * flat_pos[3 * vtx + 2]);
    }

    polyscope::init();
    let ps = polyscope::register_point_cloud("points", positions);
    polyscope::add_point_scalar_quantity(ps, "magnitude", scalars, true);
    polyscope::show();
}
