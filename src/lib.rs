extern crate libc;

mod bindings;
pub mod point_cloud_func;
pub use crate::point_cloud_func::PointCloudImpl as point_cloud;
pub use crate::point_cloud_func::ScalarFunc as pc_scalar;
pub use crate::point_cloud_func::Vec3Func as pc_vec3;

pub fn init() {
    unsafe {
        bindings::c_init();
    }
}

pub fn register_callback(function: fn()) {
    unsafe {
        bindings::c_register_callback(function as *mut libc::c_void);
    }
}

pub fn show() {
    unsafe {
        bindings::c_show();
    }
}

pub fn generate_imgui_button(name: &str) -> bool {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        return bindings::c_generate_imgui_button(c_string.as_ptr());
    }
}

pub fn generate_imgui_slider_int(name: &str, val: *mut i32, min: i32, max: i32) {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        bindings::c_generate_imgui_slider_int(
            c_string.as_ptr(),
            val as *mut libc::c_int,
            min as libc::c_int,
            max as libc::c_int,
        );
    }
}
