extern crate libc;

extern "C" {
    pub fn c_init();
    pub fn c_register_callback(function: *mut libc::c_void);
    pub fn c_register_point_cloud(
        name: *const libc::c_char,
        pts: *const [libc::c_float; 3],
        len: libc::c_int,
    ) -> *mut libc::c_void;
    pub fn c_add_point_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        values: *const libc::c_float,
        len: libc::c_int,
        enabled: bool,
    );
    pub fn c_show();

    pub fn c_generate_imgui_button(name: *const libc::c_char) -> bool;
    pub fn c_generate_imgui_slider_int(
        name: *const libc::c_char,
        val: *mut libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    );
}

pub fn init() {
    unsafe {
        c_init();
    }
}

pub fn register_callback(function: fn()) {
    unsafe {
        c_register_callback(function as *mut libc::c_void);
    }
}

pub fn register_point_cloud(name: &str, pts: &Vec<[f32; 3]>) -> *mut libc::c_void {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        return c_register_point_cloud(c_string.as_ptr(), pts.as_ptr(), pts.len() as libc::c_int);
    }
}

pub fn add_point_scalar_quantity(
    ps_point: *mut libc::c_void,
    name: &str,
    values: &Vec<f32>,
    enabled: bool,
) {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        c_add_point_scalar_quantity(
            ps_point,
            c_string.as_ptr(),
            values.as_ptr(),
            values.len() as libc::c_int,
            enabled,
        );
    }
}

pub fn show() {
    unsafe {
        c_show();
    }
}

pub fn generate_imgui_button(name: &str) -> bool {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        return c_generate_imgui_button(c_string.as_ptr());
    }
}

pub fn generate_imgui_slider_int(name: &str, val: *mut i32, min: i32, max: i32) {
    unsafe {
        let c_string = std::ffi::CString::new(name).unwrap();
        c_generate_imgui_slider_int(
            c_string.as_ptr(),
            val as *mut libc::c_int,
            min as libc::c_int,
            max as libc::c_int,
        );
    }
}
