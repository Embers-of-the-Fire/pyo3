use crate::object::*;
use std::ffi;
use std::ptr::addr_of_mut;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyRange_Type")]
    pub static mut PyRange_Type: PyTypeObject;
    pub static mut PyRangeIter_Type: PyTypeObject;
    pub static mut PyLongRangeIter_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyRange_Check(op: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(op) == addr_of_mut!(PyRange_Type)) as ffi::c_int
}
