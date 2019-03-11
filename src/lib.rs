extern crate jni_sys;

use jni_sys::*;
use std::ffi::c_void;
use std::ptr;

#[link(name = "libumfpack")]  // it is just "umfpack" on Linux
#[allow(non_snake_case)]
extern "C" {
    fn umfpack_di_symbolic(
        n_row: i32,
        n_col: i32,
        Ap: *mut i32,
        Ai: *mut i32,
        Ax: *mut f64,
        Symbolic: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    fn umfpack_di_numeric(
        Ap: *mut i32,
        Ai: *mut i32,
        Ax: *mut f64,
        Symbolic: *mut c_void,
        Numeric: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    fn umfpack_di_solve(
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

    fn umfpack_di_free_symbolic(Symbolic: *mut *mut c_void);

    fn umfpack_di_free_numeric(Numeric: *mut *mut c_void);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_umfSolve(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    columnPointers: jintArray,
    rowIndices: jintArray,
    values: jdoubleArray,
    demand: jdoubleArray,
    result: jdoubleArray,
) {
    unsafe {
        let NULL: *mut u8 = ptr::null_mut();
        let jvm = **env;

        let columnPointersPtr = jvm.GetIntArrayElements.unwrap()(env, columnPointers, NULL);
        let rowIndicesPtr = jvm.GetIntArrayElements.unwrap()(env, rowIndices, NULL);
        let valuesPtr = jvm.GetDoubleArrayElements.unwrap()(env, values, NULL);
        let demandPtr = jvm.GetDoubleArrayElements.unwrap()(env, demand, NULL);
        let resultPtr = jvm.GetDoubleArrayElements.unwrap()(env, result, NULL);

        let nullF64 = NULL as *mut f64;
        let mut Symbolic: *mut c_void = ptr::null_mut();
        let mut Numeric: *mut c_void = ptr::null_mut();

        umfpack_di_symbolic(
            n,
            n,
            columnPointersPtr,
            rowIndicesPtr,
            valuesPtr,
            &mut Symbolic,
            nullF64,
            nullF64,
        );

        umfpack_di_numeric(
            columnPointersPtr,
            rowIndicesPtr,
            valuesPtr,
            Symbolic,
            &mut Numeric,
            nullF64,
            nullF64,
        );

        umfpack_di_free_symbolic(&mut Symbolic);

        umfpack_di_solve(
            0,
            columnPointersPtr,
            rowIndicesPtr,
            valuesPtr,
            resultPtr,
            demandPtr,
            Numeric,
            nullF64,
            nullF64,
        );
        umfpack_di_free_numeric(&mut Numeric);

        jvm.ReleaseIntArrayElements.unwrap()(env, columnPointers, columnPointersPtr, 0);
        jvm.ReleaseIntArrayElements.unwrap()(env, rowIndices, rowIndicesPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, values, valuesPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, demand, demandPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, result, resultPtr, 0);
    }
}
