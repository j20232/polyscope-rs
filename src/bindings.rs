extern crate libc;

extern "C" {
    pub fn c_init();
    pub fn c_register_callback(function: *mut libc::c_void);

    pub fn c_register_float_point_cloud(
        name: *const libc::c_char,
        pts: *const [libc::c_float; 3],
        len: libc::c_int,
    ) -> *mut libc::c_void;

    pub fn c_register_double_point_cloud(
        name: *const libc::c_char,
        pts: *const [libc::c_double; 3],
        len: libc::c_int,
    ) -> *mut libc::c_void;

    pub fn c_add_float_point_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        values: *const libc::c_float,
        len: libc::c_int,
        enabled: bool,
    );

    pub fn c_add_double_point_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        values: *const libc::c_double,
        len: libc::c_int,
        enabled: bool,
    );

    pub fn c_add_float_point_color_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        colors: *const [libc::c_float; 3],
        len: libc::c_int,
        enabled: bool,
    );

    pub fn c_add_double_point_color_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        colors: *const [libc::c_double; 3],
        len: libc::c_int,
        enabled: bool,
    );

    pub fn c_add_float_point_vector_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        vecs: *const [libc::c_float; 3],
        len: libc::c_int,
        enabled: bool,
    );

    pub fn c_add_double_point_vector_quantity(
        ps_point: *mut libc::c_void,
        name: *const libc::c_char,
        vecs: *const [libc::c_double; 3],
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
