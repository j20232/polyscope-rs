extern crate libc;

use crate::bindings;

pub fn button(name: &str) -> bool {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        return bindings::c_generate_imgui_button(c_string.as_ptr());
    }
}

pub fn slider_int(name: &str, val: *mut i32, min: i32, max: i32) {
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
