use crate::object::*;
use std::ffi;
use std::ptr::addr_of_mut;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_Type")]
    pub static mut PyCapsule_Type: PyTypeObject;
}

pub type PyCapsule_Destructor = unsafe extern "C" fn(o: *mut PyObject);

#[inline]
pub unsafe fn PyCapsule_CheckExact(ob: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(ob) == addr_of_mut!(PyCapsule_Type)) as ffi::c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_New")]
    pub fn PyCapsule_New(
        pointer: *mut ffi::c_void,
        name: *const ffi::c_char,
        destructor: Option<PyCapsule_Destructor>,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_GetPointer")]
    pub fn PyCapsule_GetPointer(
        capsule: *mut PyObject,
        name: *const ffi::c_char,
    ) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_GetDestructor")]
    pub fn PyCapsule_GetDestructor(capsule: *mut PyObject) -> Option<PyCapsule_Destructor>;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_GetName")]
    pub fn PyCapsule_GetName(capsule: *mut PyObject) -> *const ffi::c_char;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_GetContext")]
    pub fn PyCapsule_GetContext(capsule: *mut PyObject) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_IsValid")]
    pub fn PyCapsule_IsValid(capsule: *mut PyObject, name: *const ffi::c_char) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_SetPointer")]
    pub fn PyCapsule_SetPointer(capsule: *mut PyObject, pointer: *mut ffi::c_void) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_SetDestructor")]
    pub fn PyCapsule_SetDestructor(
        capsule: *mut PyObject,
        destructor: Option<PyCapsule_Destructor>,
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_SetName")]
    pub fn PyCapsule_SetName(capsule: *mut PyObject, name: *const ffi::c_char) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_SetContext")]
    pub fn PyCapsule_SetContext(capsule: *mut PyObject, context: *mut ffi::c_void) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCapsule_Import")]
    pub fn PyCapsule_Import(name: *const ffi::c_char, no_block: ffi::c_int) -> *mut ffi::c_void;
}
