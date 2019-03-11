#[link(name = "libopenblas64_")]
#[allow(non_snake_case)]
extern "C" {
    pub fn dgemv64_(
        TRANS: *mut char,
        M: *mut i64,
        N: *mut i64,
        ALPHA: *mut f64,
        A: *mut f64,
        LDA: *mut i64,
        X: *mut f64,
        INCX: *mut i64,
        BETA: *mut f64,
        Y: *mut f64,
        INCY: *mut i64);
}
