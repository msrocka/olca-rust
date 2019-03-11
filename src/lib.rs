extern crate jni_sys;

use jni_sys::*;
use std::ffi::c_void;
use std::ptr;

mod umf;
mod blas;

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

        umf::umfpack_di_symbolic(
            n,
            n,
            columnPointersPtr,
            rowIndicesPtr,
            valuesPtr,
            &mut Symbolic,
            nullF64,
            nullF64,
        );

        umf::umfpack_di_numeric(
            columnPointersPtr,
            rowIndicesPtr,
            valuesPtr,
            Symbolic,
            &mut Numeric,
            nullF64,
            nullF64,
        );

        umf::umfpack_di_free_symbolic(&mut Symbolic);

        umf::umfpack_di_solve(
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
        umf::umfpack_di_free_numeric(&mut Numeric);

        jvm.ReleaseIntArrayElements.unwrap()(env, columnPointers, columnPointersPtr, 0);
        jvm.ReleaseIntArrayElements.unwrap()(env, rowIndices, rowIndicesPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, values, valuesPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, demand, demandPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, result, resultPtr, 0);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_mvmult(
    env: *mut JNIEnv,
    _class: jclass,
    rowsA: jint,
    colsA: jint,
    a: jdoubleArray,
    x: jdoubleArray,
    y: jdoubleArray) {

    unsafe {
        let NULL: *mut u8 = ptr::null_mut();
        let jvm = **env;

        let aPtr = jvm.GetDoubleArrayElements.unwrap()(env, a, NULL);
        let xPtr = jvm.GetDoubleArrayElements.unwrap()(env, x, NULL);
        let yPtr = jvm.GetDoubleArrayElements.unwrap()(env, y, NULL);

        let mut trans = 'N';
        let mut alpha:f64 = 1.0;
        let mut beta:f64 = 0.0;
        let mut inc:i64 = 1;
        let mut rowsA_64:i64 = rowsA as i64;
        let mut colsA_64:i64 = colsA as i64;

        blas::dgemv(
            &mut trans,
            &mut rowsA_64,
            &mut colsA_64,
            &mut alpha,
            aPtr,
            &mut rowsA_64,
            xPtr,
            &mut inc,
            &mut beta,
            yPtr,
            &mut inc);

        jvm.ReleaseDoubleArrayElements.unwrap()(env, a, aPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, x, xPtr, 0);
        jvm.ReleaseDoubleArrayElements.unwrap()(env, y, yPtr, 0);
    }
}
