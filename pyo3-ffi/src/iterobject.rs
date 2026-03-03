use crate::object::*;
use std::ffi;
use std::ptr::addr_of_mut;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PySeqIter_Type: PyTypeObject;
    pub static mut PyCallIter_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PySeqIter_Check(op: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(op) == addr_of_mut!(PySeqIter_Type)) as ffi::c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPySeqIter_New")]
    pub fn PySeqIter_New(arg1: *mut PyObject) -> *mut PyObject;
}

#[inline]
pub unsafe fn PyCallIter_Check(op: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(op) == addr_of_mut!(PyCallIter_Type)) as ffi::c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyCallIter_New")]
    pub fn PyCallIter_New(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
}
