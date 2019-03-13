extern crate jni_sys;
extern crate libc;

use std::ptr;
use std::ffi::c_void;

use libc::{c_char, malloc, free};
use jni_sys::*;

mod umf;
mod blas;

const NULL: *mut u8 = ptr::null_mut();

/*
unsafe fn get_array_i32(env: *mut JNIEnv, array: &mut jintArray) -> *mut i32{
    return (**env).GetIntArrayElements.unwrap()(env, *array, NULL);
}
*/

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
    return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_f64(env: *mut JNIEnv, array: jdoubleArray, ptr: *mut f64) {
    (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

#[no_mangle]
#[cfg(umfpack)]
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
        // let NULL: *mut u8 = ptr::null_mut();
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

/// Performs a dense matrix-vector multiplication of a `m` by `n` matrix `A`
/// with a vector `x`: `y = A * x`.
/// 
/// * `m` - The number of rows of the matrix `A`
/// * `n` - The number of columns of the matrix `A`
/// * `A` - The matrix `A` stored in column-major order in an array.
/// * `x` - The vector `x` as an array of size `n`.
/// * `y` - The result vector `y` as an initialized array of size `m`.
/// 
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_mvmult(
    env: *mut JNIEnv,
    _class: jclass,
    m: jint,
    n: jint,
    A: jdoubleArray,
    x: jdoubleArray,
    y: jdoubleArray) {
    unsafe {
        let aPtr = get_array_f64(env, A);
        let xPtr = get_array_f64(env, x);
        let yPtr = get_array_f64(env, y);

        let mut trans = 'N' as c_char;
        let mut alpha: f64 = 1.0;
        let mut beta: f64 = 0.0;
        let mut inc: i64 = 1;
        let mut rowsA_64: i64 = m as i64;
        let mut colsA_64: i64 = n as i64;

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

        release_array_f64(env, A, aPtr);
        release_array_f64(env, x, xPtr);
        release_array_f64(env, y, yPtr);
    }
}

/// Performs a dense matrix-matrix multiplication: `C := A * B`
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_mmult(
    env: *mut JNIEnv,
    _class: jclass,
    rowsA: jint,
    colsB: jint,
    k: jint,
    A: jdoubleArray,
    B: jdoubleArray,
    C: jdoubleArray) {
    unsafe {
        let ptrA = get_array_f64(env, A);
        let ptrB = get_array_f64(env, B);
        let ptrC = get_array_f64(env, C);

        let mut trans = 'N' as c_char;
        let mut alpha: f64 = 1.0;
        let mut beta: f64 = 0.0;
        let mut rowsA_64 = rowsA as i64;
        let mut colsB_64 = colsB as i64;
        let mut k_64 = k as i64;

        blas::dgemm(
            &mut trans,
            &mut trans,
            &mut rowsA_64,
            &mut colsB_64,
            &mut k_64,
            &mut alpha,
            ptrA,
            &mut rowsA_64,
            ptrB,
            &mut k_64,
            &mut beta,
            ptrC,
            &mut rowsA_64,
        );

        release_array_f64(env, A, ptrA);
        release_array_f64(env, B, ptrB);
        release_array_f64(env, C, ptrC);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solve(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    nrhs: jint,
    A: jdoubleArray,
    B: jdoubleArray) -> jint {
    unsafe {
        let ptrA = get_array_f64(env, A);
        let ptrB = get_array_f64(env, B);

        let mut n_64 = n as i64;
        let mut nrhs_64 = nrhs as i64;
        let ipiv = malloc((8 * n) as usize) as *mut i64;
        let mut info: i64 = 0;

        blas::dgesv(
            &mut n_64,
            &mut nrhs_64,
            ptrA,
            &mut n_64,
            ipiv,
            ptrB,
            &mut n_64,
            &mut info);

        free(ipiv as *mut c_void);
        release_array_f64(env, A, ptrA);
        release_array_f64(env, B, ptrB);

        return info as jint;
    }
}