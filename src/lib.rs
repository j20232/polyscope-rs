extern crate libc;

mod bindings;
pub mod imgui;
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
