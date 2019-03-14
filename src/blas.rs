extern crate libc;

use libc::c_char;

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

    /// [DGESV](http://www.netlib.org/lapack/explore-html/d8/d72/dgesv_8f.html)
    /// computes the solution to system of linear equations `A * X = B` for
    /// GE matrices
    #[cfg_attr(target_os = "windows", link_name = "dgesv64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgesv_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgesv_64_")]
    pub fn dgesv(
        N: *mut i64,
        NRHS: *mut i64,
        A: *mut f64,
        LDA: *mut i64,
        IPIV: *mut i64,
        B: *mut f64,
        LDB: *mut i64,
        INFO: *mut i64,
    );

    /// [DGETRF](http://www.netlib.org/lapack/explore-html/d3/d6a/dgetrf_8f.html)
    /// computes an LU factorization of a general M-by-N matrix A using partial
    /// pivoting with row interchanges.
    #[cfg_attr(target_os = "windows", link_name = "dgetrf64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgetrf_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgetrf_64_")]
    pub fn dgetrf(
        M: *mut i64,
        N: *mut i64,
        A: *mut f64,
        LDA: *mut i64,
        IPIV: *mut i64,
        INFO: *mut i64,
    );

    /// [DGETRI](http://www.netlib.org/lapack/explore-html/df/da4/dgetri_8f.html)
    /// DGETRI computes the inverse of a matrix using the LU factorization
    /// computed by DGETRF.
    #[cfg_attr(target_os = "windows", link_name = "dgetri64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgetri_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgetri_64_")]
    pub fn dgetri(
        N: *mut i64,
        A: *mut f64,
        LDA: *mut i64,
        IPIV: *mut i64,
        WORK: *mut f64,
        LWORK: *mut i64,
        INFO: *mut i64,        
    );
}
