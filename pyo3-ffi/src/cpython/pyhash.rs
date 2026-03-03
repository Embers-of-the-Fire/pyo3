#[cfg(Py_3_14)]
use crate::Py_ssize_t;
#[cfg(Py_3_13)]
use crate::{PyObject, Py_hash_t};
#[cfg(not(PyPy))]
use std::ffi;

#[cfg(not(PyPy))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyHash_FuncDef {
    pub hash: Option<
        extern "C" fn(arg1: *const ffi::c_void, arg2: crate::Py_ssize_t) -> crate::Py_hash_t,
    >,
    pub name: *const ffi::c_char,
    pub hash_bits: ffi::c_int,
    pub seed_bits: ffi::c_int,
}

#[cfg(not(PyPy))]
impl Default for PyHash_FuncDef {
    #[inline]
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

extern "C" {
    #[cfg(not(PyPy))]
    pub fn PyHash_GetFuncDef() -> *mut PyHash_FuncDef;
    #[cfg(Py_3_13)]
    pub fn Py_HashPointer(ptr: *const ffi::c_void) -> Py_hash_t;
    #[cfg(Py_3_13)]
    pub fn PyObject_GenericHash(obj: *mut PyObject) -> Py_hash_t;
    #[cfg(Py_3_14)]
    pub fn Py_HashBuffer(ptr: *const ffi::c_void, len: Py_ssize_t) -> Py_hash_t;
}
