extern crate libc;

use crate::bindings;

pub struct PointCloudImpl;

pub trait Vec3Func<T> {
    fn register_point_cloud(name: &str, pts: &Vec<[T; 3]>) -> *mut libc::c_void;
    fn add_color_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        colors: &Vec<[T; 3]>,
        enabled: bool,
    );
    fn add_vector_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        vecs: &Vec<[T; 3]>,
        enabled: bool,
    );
}

pub trait ScalarFunc<T> {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        scala_values: &Vec<T>,
        enabled: bool,
    );
}

impl Vec3Func<f32> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &Vec<[f32; 3]>) -> *mut libc::c_void {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_float_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr(),
                pts.len() as libc::c_int,
            );
        }
    }

    fn add_color_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        colors: &Vec<[f32; 3]>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_float_point_color_quantity(
                ps_point,
                c_string.as_ptr(),
                colors.as_ptr(),
                colors.len() as libc::c_int,
                enabled,
            );
        }
    }

    fn add_vector_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        vecs: &Vec<[f32; 3]>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_float_point_vector_quantity(
                ps_point,
                c_string.as_ptr(),
                vecs.as_ptr(),
                vecs.len() as libc::c_int,
                enabled,
            );
        }
    }
}

impl Vec3Func<f64> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &Vec<[f64; 3]>) -> *mut libc::c_void {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_double_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr(),
                pts.len() as libc::c_int,
            );
        }
    }

    fn add_color_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        colors: &Vec<[f64; 3]>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_double_point_color_quantity(
                ps_point,
                c_string.as_ptr(),
                colors.as_ptr(),
                colors.len() as libc::c_int,
                enabled,
            );
        }
    }

    fn add_vector_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        vecs: &Vec<[f64; 3]>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_double_point_vector_quantity(
                ps_point,
                c_string.as_ptr(),
                vecs.as_ptr(),
                vecs.len() as libc::c_int,
                enabled,
            );
        }
    }
}

impl ScalarFunc<f32> for PointCloudImpl {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        values: &Vec<f32>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            bindings::c_add_float_point_scalar_quantity(
                ps_point,
                c_string.as_ptr(),
                values.as_ptr(),
                values.len() as libc::c_int,
                enabled,
            );
        }
    }
}

impl ScalarFunc<f64> for PointCloudImpl {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        values: &Vec<f64>,
        enabled: bool,
    ) {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            bindings::c_add_double_point_scalar_quantity(
                ps_point,
                c_string.as_ptr(),
                values.as_ptr(),
                values.len() as libc::c_int,
                enabled,
            );
        }
    }
}
