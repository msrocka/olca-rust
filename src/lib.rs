extern crate jni_sys;
extern crate libc;

use std::ffi::c_void;
use std::ptr;

use jni_sys::*;
use libc::{c_char, free, malloc, memcpy};

mod blas;
mod umf;

const NULL: *mut u8 = ptr::null_mut();

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
    return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_f64(
    env: *mut JNIEnv,
    array: jdoubleArray,
    ptr: *mut f64,
) {
    (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

/// Get the raw pointer of the given array from the JVM.
#[cfg(umfpack)]
unsafe fn get_array_i32(env: *mut JNIEnv, array: jintArray) -> *mut i32 {
    return (**env).GetIntArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
#[cfg(umfpack)]
unsafe fn release_array_i32(env: *mut JNIEnv, array: jintArray, ptr: *mut i32) {
    (**env).ReleaseIntArrayElements.unwrap()(env, array, ptr, 0);
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
        let columnPointersPtr = get_array_i32(env, columnPointers);
        let rowIndicesPtr = get_array_i32(env, rowIndices);
        let valuesPtr = get_array_f64(env, values);
        let demandPtr = get_array_f64(env, demand);
        let resultPtr = get_array_f64(env, result);

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

        release_array_i32(env, columnPointers, columnPointersPtr);
        release_array_i32(env, rowIndices, rowIndicesPtr);
        release_array_f64(env, values, valuesPtr);
        release_array_f64(env, demand, demandPtr);
        release_array_f64(env, result, resultPtr);
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
    y: jdoubleArray,
) {
    unsafe {
        let aPtr = get_array_f64(env, A);
        let xPtr = get_array_f64(env, x);
        let yPtr = get_array_f64(env, y);

        let trans = 'N' as c_char;
        let alpha: f64 = 1.0;
        let beta: f64 = 0.0;
        let inc: i64 = 1;
        let rowsA_64: i64 = m as i64;
        let colsA_64: i64 = n as i64;

        blas::dgemv(
            &trans, &rowsA_64, &colsA_64, &alpha, aPtr, &rowsA_64, xPtr, &inc,
            &beta, yPtr, &inc,
        );

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
    C: jdoubleArray,
) {
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
    B: jdoubleArray,
) -> jint {
    unsafe {
        let ptrA = get_array_f64(env, A);
        let ptrB = get_array_f64(env, B);

        let n_64 = n as i64;
        let nrhs_64 = nrhs as i64;
        let ipiv = malloc((8 * n) as usize) as *mut i64;
        let mut info: i64 = 0;

        blas::dgesv(&n_64, &nrhs_64, ptrA, &n_64, ipiv, ptrB, &n_64, &mut info);

        free(ipiv as *mut c_void);
        release_array_f64(env, A, ptrA);
        release_array_f64(env, B, ptrB);

        return info as jint;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_invert(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    A: jdoubleArray,
) -> jint {
    unsafe {
        let mut n_64 = n as i64;
        let ptrA = get_array_f64(env, A);
        let ipiv = malloc((8 * n) as usize) as *mut i64;
        let mut info: i64 = 0;

        // calculate the factorization
        blas::dgetrf(&n_64, &n_64, ptrA, &n_64, ipiv, &mut info);

        if info != 0 {
            // factorization error
            free(ipiv as *mut c_void);
            release_array_f64(env, A, ptrA);
            return info as jint;
        }

        let mut lwork = (64 * 2 * n) as i64;
        let work = malloc((8 * lwork) as usize) as *mut f64;

        // invert it
        blas::dgetri(
            &mut n_64, ptrA, &mut n_64, ipiv, work, &mut lwork, &mut info,
        );

        free(ipiv as *mut c_void);
        free(work as *mut c_void);
        release_array_f64(env, A, ptrA);

        return info as jint;
    }
}

struct DenseFactorization {
    n: i64,
    matrix: *mut f64,
    pivot_indices: *mut i64,
}

impl Drop for DenseFactorization {
    fn drop(&mut self) {
        unsafe {
            free(self.matrix as *mut c_void);
            free(self.pivot_indices as *mut c_void);
        }
    }
}

/// Computes the LU factorization of the given dense matrix. The given matrix
/// is not modified. This function returns a pointer to the calculated
/// factorization.
#[no_mangle]
pub extern "C" fn create_dense_factorization(
    n: i64,
    matrix: *const f64,
) -> i64 {
    unsafe {
        let byte_count = (n * n * 8) as usize;
        let factorization = malloc(byte_count) as *mut f64;
        memcpy(
            factorization as *mut c_void,
            matrix as *const c_void,
            byte_count,
        );
        let pivot_indices = malloc((8 * n) as usize) as *mut i64;
        let mut info = 0i64;
        blas::dgetrf(&n, &n, factorization, &n, pivot_indices, &mut info);

        let ptr = Box::into_raw(Box::new(DenseFactorization {
            n,
            matrix: factorization,
            pivot_indices,
        }));
        return ptr as i64;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_createDenseFactorization(
    env: *mut JNIEnv,
    __class: jclass,
    n: jint,
    matrix: jdoubleArray,
) -> jlong {
    unsafe {
        let matrix_ptr = get_array_f64(env, matrix);
        let factorization_ptr =
            create_dense_factorization(n as i64, matrix_ptr);
        release_array_f64(env, matrix, matrix_ptr);
        return factorization_ptr;
    }
}

#[no_mangle]
pub extern "C" fn solve_dense_factorization(
    factorization: i64,
    columns: i64,
    b: *mut f64,
) {
    unsafe {
        let f = factorization as *const DenseFactorization;
        let n = (*f).n;
        let mut info: i64 = 0;
        blas::dgetrs(
            &('N' as c_char),
            &n,
            &columns,
            (*f).matrix,
            &n,
            (*f).pivot_indices,
            b,
            &n,
            &mut info,
        );
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solveDenseFactorization(
    env: *mut JNIEnv,
    __class: jclass,
    factorization: jlong,
    columns: jint,
    b: jdoubleArray,
) {
    unsafe {
        let b_ptr = get_array_f64(env, b);
        solve_dense_factorization(factorization, columns as i64, b_ptr);
        release_array_f64(env, b, b_ptr);
    }
}

#[no_mangle]
pub extern "C" fn destroy_dense_factorization(ptr: i64) {
    unsafe {
        let p = ptr as *mut DenseFactorization;
        let f = Box::from_raw(p);
        drop(f);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_destroyDenseFactorization(
    _env: *mut JNIEnv,
    __class: jclass,
    factorization: jlong,
) {
    destroy_dense_factorization(factorization);
}
