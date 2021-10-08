extern crate polyscope;

fn main() {
    unsafe {
        polyscope::init();
        polyscope::show();
    }
}
