#[cfg(not(GraalPy))]
use crate::object::*;
#[cfg(not(GraalPy))]
use crate::PyCodeObject;
use crate::PyFrameObject;
#[cfg(not(GraalPy))]
use crate::PyThreadState;
use std::ffi;

#[cfg(not(any(PyPy, GraalPy, Py_3_11)))]
pub type PyFrameState = ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(any(PyPy, GraalPy, Py_3_11)))]
pub struct PyTryBlock {
    pub b_type: ffi::c_int,
    pub b_handler: ffi::c_int,
    pub b_level: ffi::c_int,
}

// skipped _PyFrame_IsRunnable
// skipped _PyFrame_IsExecuting
// skipped _PyFrameHasCompleted

extern "C" {
    #[cfg(not(GraalPy))]
    #[cfg_attr(PyPy, link_name = "PyPyFrame_New")]
    pub fn PyFrame_New(
        tstate: *mut PyThreadState,
        code: *mut PyCodeObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
    ) -> *mut PyFrameObject;
    // skipped _PyFrame_New_NoTrack

    pub fn PyFrame_BlockSetup(
        f: *mut PyFrameObject,
        _type: ffi::c_int,
        handler: ffi::c_int,
        level: ffi::c_int,
    );
    #[cfg(not(any(PyPy, GraalPy, Py_3_11)))]
    pub fn PyFrame_BlockPop(f: *mut PyFrameObject) -> *mut PyTryBlock;

    pub fn PyFrame_LocalsToFast(f: *mut PyFrameObject, clear: ffi::c_int);
    pub fn PyFrame_FastToLocalsWithError(f: *mut PyFrameObject) -> ffi::c_int;
    pub fn PyFrame_FastToLocals(f: *mut PyFrameObject);

    // skipped _PyFrame_DebugMallocStats

    #[cfg(not(Py_3_9))]
    pub fn PyFrame_ClearFreeList() -> ffi::c_int;
}
