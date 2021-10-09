use cmake;

#[cfg(target_os = "windows")]
fn build() {
    println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=shell32");
}

#[cfg(target_os = "linux")]
fn build() {}

fn main() {
    let dst = cmake::build("src/cpp");
    println!("cargo:rustc-link-search=native={}", dst.display());
    build();
    println!("cargo:rustc-link-lib=static=rust_polyscope");
    println!("cargo:rustc-link-lib=static=polyscope");
    println!("cargo:rustc-link-lib=static=glad");
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=static=imgui");
    println!("cargo:rustc-link-lib=static=stb");
}
