extern crate libc;

use libc::c_char;

// #[link(name = "libopenblas64_")]

#[allow(non_snake_case)]
#[cfg_attr(target_os = "windows", link(name = "libopenblas64_"))]
#[cfg_attr(target_os = "linux", link(name = "openblas64_"))]
#[cfg_attr(target_os = "macos", link(name = "openblas64_"))]
extern "C" {

    /// [DGEMV](http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html)
    ///  performs one of the matrix-vector operations
    /// `y := alpha*A*x + beta*y`   or   `y := alpha*A**T*x + beta*y`
    /// where `alpha` and `beta` are scalars, `x` and `y` are vectors and `A`
    /// is an `m` by `n` matrix.
    #[cfg_attr(target_os = "windows", link_name = "dgemv64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgemv_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgemv_64_")]
    pub fn dgemv(
        TRANS: *mut c_char,
        M: *mut i64,
        N: *mut i64,
        ALPHA: *mut f64,
        A: *mut f64,
        LDA: *mut i64,
        X: *mut f64,
        INCX: *mut i64,
        BETA: *mut f64,
        Y: *mut f64,
        INCY: *mut i64,
    );

    /// [DGEMM](http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html)
    /// performs one of the matrix-matrix operations
    ///
    /// `C := alpha*op( A )*op( B ) + beta*C`,
    ///
    /// where  `op( X )` is one of
    ///
    /// `op( X ) = X`   or   `op( X ) = X**T`,
    ///
    /// `alpha` and `beta` are scalars, and `A`, `B` and `C` are matrices,
    /// with `op( A )` an `m` by `k` matrix,  `op( B )`  a  `k` by `n`
    /// matrix and `C` an `m` by `n` matrix.
    #[cfg_attr(target_os = "windows", link_name = "dgemm64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgemm_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgemm_64_")]
    pub fn dgemm(
        TRANSA: *mut c_char,
        TRANSB: *mut c_char,
        M: *mut i64,
        N: *mut i64,
        K: *mut i64,
        ALPHA: *mut f64,
        A: *mut f64,
        LDA: *mut i64,
        B: *mut f64,
        LDB: *mut i64,
        BETA: *mut f64,
        C: *mut f64,
        LDC: *mut i64,
    );
}
