extern crate jni_sys;

use jni_sys::*;
use std::ffi::c_void;
use std::ptr;

mod umf;

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
