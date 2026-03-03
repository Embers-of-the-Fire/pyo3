use crate::object::PyObject;
use crate::pyport::Py_ssize_t;
use std::ffi;

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyErr_WarnEx")]
    pub fn PyErr_WarnEx(
        category: *mut PyObject,
        message: *const ffi::c_char,
        stack_level: Py_ssize_t,
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyErr_WarnFormat")]
    pub fn PyErr_WarnFormat(
        category: *mut PyObject,
        stack_level: Py_ssize_t,
        format: *const ffi::c_char,
        ...
    ) -> ffi::c_int;
    pub fn PyErr_ResourceWarning(
        source: *mut PyObject,
        stack_level: Py_ssize_t,
        format: *const ffi::c_char,
        ...
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyErr_WarnExplicit")]
    pub fn PyErr_WarnExplicit(
        category: *mut PyObject,
        message: *const ffi::c_char,
        filename: *const ffi::c_char,
        lineno: ffi::c_int,
        module: *const ffi::c_char,
        registry: *mut PyObject,
    ) -> ffi::c_int;
}
