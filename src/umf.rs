use std::ffi::c_void;

// #[link(name = "libumfpack")]

#[cfg(umfpack)]
#[allow(non_snake_case)]
#[cfg_attr(target_os = "windows", link(name = "libumfpack"))]
#[cfg_attr(target_os = "linux", link(name = "umfpack"))]
#[cfg_attr(target_os = "macos", link(name = "umfpack"))]
extern "C" {
    pub fn umfpack_di_symbolic(
        n_row: i32,
        n_col: i32,
        Ap: *mut i32,
        Ai: *mut i32,
        Ax: *mut f64,
        Symbolic: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_numeric(
        Ap: *mut i32,
        Ai: *mut i32,
        Ax: *mut f64,
        Symbolic: *mut c_void,
        Numeric: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_solve(
        sys: i32,
        Ap: *mut i32,
        Ai: *mut i32,
        Ax: *mut f64,
        X: *mut f64,
        B: *mut f64,
        Numeric: *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_free_symbolic(Symbolic: *mut *mut c_void);

    pub fn umfpack_di_free_numeric(Numeric: *mut *mut c_void);
}