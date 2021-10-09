extern crate libc;
extern crate nalgebra as na;

use crate::bindings;

pub struct PointCloudImpl;

pub trait Vec3Func<T> {
    fn register_point_cloud(name: &str, pts: &T) -> *mut libc::c_void;
    fn add_color_quantity(ps_point: *mut libc::c_void, name: &str, colors: &T, enabled: bool);
    fn add_vector_quantity(ps_point: *mut libc::c_void, name: &str, vecs: &T, enabled: bool);
}

pub trait ScalarFunc<T> {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        scala_values: &T,
        enabled: bool,
    );
}

// ---------------------- Vec3Func ----------------------

// nalgebra dynamic float
impl Vec3Func<na::DMatrix<f32>> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &na::DMatrix<f32>) -> *mut libc::c_void {
        assert!(pts.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_float_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr() as *const libc::c_float,
                pts.nrows() as libc::c_int,
            );
        }
    }

    fn add_color_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        colors: &na::DMatrix<f32>,
        enabled: bool,
    ) {
        assert!(colors.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_float_point_color_quantity(
                ps_point,
                c_string.as_ptr(),
                colors.as_ptr() as *const libc::c_float,
                colors.nrows() as libc::c_int,
                enabled,
            );
        }
    }

    fn add_vector_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        vecs: &na::DMatrix<f32>,
        enabled: bool,
    ) {
        assert!(vecs.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_float_point_vector_quantity(
                ps_point,
                c_string.as_ptr(),
                vecs.as_ptr() as *const libc::c_float,
                vecs.nrows() as libc::c_int,
                enabled,
            );
        }
    }
}

// nalgebra dynamic double
impl Vec3Func<na::DMatrix<f64>> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &na::DMatrix<f64>) -> *mut libc::c_void {
        assert!(pts.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_double_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr() as *const libc::c_double,
                pts.nrows() as libc::c_int,
            );
        }
    }

    fn add_color_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        colors: &na::DMatrix<f64>,
        enabled: bool,
    ) {
        assert!(colors.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_double_point_color_quantity(
                ps_point,
                c_string.as_ptr(),
                colors.as_ptr() as *const libc::c_double,
                colors.nrows() as libc::c_int,
                enabled,
            );
        }
    }

    fn add_vector_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        vecs: &na::DMatrix<f64>,
        enabled: bool,
    ) {
        assert!(vecs.ncols() == 3);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_add_double_point_vector_quantity(
                ps_point,
                c_string.as_ptr(),
                vecs.as_ptr() as *const libc::c_double,
                vecs.nrows() as libc::c_int,
                enabled,
            );
        }
    }
}

// Vec<[f32; 3]> float
impl Vec3Func<Vec<[f32; 3]>> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &Vec<[f32; 3]>) -> *mut libc::c_void {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_float_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr() as *const libc::c_float,
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
                colors.as_ptr() as *const libc::c_float,
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
                vecs.as_ptr() as *const libc::c_float,
                vecs.len() as libc::c_int,
                enabled,
            );
        }
    }
}

// Vec<[f64; 3]> double
impl Vec3Func<Vec<[f64; 3]>> for PointCloudImpl {
    fn register_point_cloud(name: &str, pts: &Vec<[f64; 3]>) -> *mut libc::c_void {
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            return bindings::c_register_double_point_cloud(
                c_string.as_ptr(),
                pts.as_ptr() as *const libc::c_double,
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
                colors.as_ptr() as *const libc::c_double,
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
                vecs.as_ptr() as *const libc::c_double,
                vecs.len() as libc::c_int,
                enabled,
            );
        }
    }
}

// ---------------------- ScalarFunc ----------------------
impl ScalarFunc<na::DMatrix<f32>> for PointCloudImpl {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        values: &na::DMatrix<f32>,
        enabled: bool,
    ) {
        assert!(values.ncols() == 1);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            bindings::c_add_float_point_scalar_quantity(
                ps_point,
                c_string.as_ptr(),
                values.as_ptr(),
                values.nrows() as libc::c_int,
                enabled,
            );
        }
    }
}

impl ScalarFunc<Vec<f32>> for PointCloudImpl {
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

impl ScalarFunc<na::DMatrix<f64>> for PointCloudImpl {
    fn add_scalar_quantity(
        ps_point: *mut libc::c_void,
        name: &str,
        values: &na::DMatrix<f64>,
        enabled: bool,
    ) {
        assert!(values.ncols() == 1);
        unsafe {
            let c_string = std::ffi::CString::new(name).unwrap();
            bindings::c_add_double_point_scalar_quantity(
                ps_point,
                c_string.as_ptr(),
                values.as_ptr(),
                values.nrows() as libc::c_int,
                enabled,
            );
        }
    }
}

impl ScalarFunc<Vec<f64>> for PointCloudImpl {
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
