use crate::methodobject::PyMethodDef;
use crate::object::{PyObject, PyTypeObject};
use crate::Py_ssize_t;
use std::ffi;
use std::ptr;

pub type getter =
    unsafe extern "C" fn(slf: *mut PyObject, closure: *mut ffi::c_void) -> *mut PyObject;
pub type setter = unsafe extern "C" fn(
    slf: *mut PyObject,
    value: *mut PyObject,
    closure: *mut ffi::c_void,
) -> ffi::c_int;

/// Represents the [PyGetSetDef](https://docs.python.org/3/c-api/structures.html#c.PyGetSetDef)
/// structure.
///
/// Note that CPython may leave fields uninitialized. You must ensure that
/// `name` != NULL before dereferencing or reading other fields.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PyGetSetDef {
    pub name: *const ffi::c_char,
    pub get: Option<getter>,
    pub set: Option<setter>,
    pub doc: *const ffi::c_char,
    pub closure: *mut ffi::c_void,
}

impl Default for PyGetSetDef {
    fn default() -> PyGetSetDef {
        PyGetSetDef {
            name: ptr::null(),
            get: None,
            set: None,
            doc: ptr::null(),
            closure: ptr::null_mut(),
        }
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyClassMethodDescr_Type")]
    pub static mut PyClassMethodDescr_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyGetSetDescr_Type")]
    pub static mut PyGetSetDescr_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyMemberDescr_Type")]
    pub static mut PyMemberDescr_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyMethodDescr_Type")]
    pub static mut PyMethodDescr_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyWrapperDescr_Type")]
    pub static mut PyWrapperDescr_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyDictProxy_Type")]
    pub static mut PyDictProxy_Type: PyTypeObject;
    #[cfg_attr(PyPy, link_name = "PyPyProperty_Type")]
    pub static mut PyProperty_Type: PyTypeObject;
}

extern "C" {
    pub fn PyDescr_NewMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDescr_NewClassMethod")]
    pub fn PyDescr_NewClassMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef)
        -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDescr_NewMember")]
    pub fn PyDescr_NewMember(arg1: *mut PyTypeObject, arg2: *mut PyMemberDef) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDescr_NewGetSet")]
    pub fn PyDescr_NewGetSet(arg1: *mut PyTypeObject, arg2: *mut PyGetSetDef) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyDictProxy_New")]
    pub fn PyDictProxy_New(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyWrapper_New(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
}

/// Represents the [PyMemberDef](https://docs.python.org/3/c-api/structures.html#c.PyMemberDef)
/// structure.
///
/// Note that CPython may leave fields uninitialized. You must always ensure that
/// `name` != NULL before dereferencing or reading other fields.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PyMemberDef {
    pub name: *const ffi::c_char,
    pub type_code: ffi::c_int,
    pub offset: Py_ssize_t,
    pub flags: ffi::c_int,
    pub doc: *const ffi::c_char,
}

impl Default for PyMemberDef {
    fn default() -> PyMemberDef {
        PyMemberDef {
            name: ptr::null_mut(),
            type_code: 0,
            offset: 0,
            flags: 0,
            doc: ptr::null_mut(),
        }
    }
}

/* Types */
pub const Py_T_SHORT: ffi::c_int = 0;
pub const Py_T_INT: ffi::c_int = 1;
pub const Py_T_LONG: ffi::c_int = 2;
pub const Py_T_FLOAT: ffi::c_int = 3;
pub const Py_T_DOUBLE: ffi::c_int = 4;
pub const Py_T_STRING: ffi::c_int = 5;
#[deprecated(note = "Use Py_T_OBJECT_EX instead")]
pub const _Py_T_OBJECT: ffi::c_int = 6;
pub const Py_T_CHAR: ffi::c_int = 7;
pub const Py_T_BYTE: ffi::c_int = 8;
pub const Py_T_UBYTE: ffi::c_int = 9;
pub const Py_T_USHORT: ffi::c_int = 10;
pub const Py_T_UINT: ffi::c_int = 11;
pub const Py_T_ULONG: ffi::c_int = 12;
pub const Py_T_STRING_INPLACE: ffi::c_int = 13;
pub const Py_T_BOOL: ffi::c_int = 14;
pub const Py_T_OBJECT_EX: ffi::c_int = 16;
pub const Py_T_LONGLONG: ffi::c_int = 17;
pub const Py_T_ULONGLONG: ffi::c_int = 18;
pub const Py_T_PYSSIZET: ffi::c_int = 19;
#[deprecated(note = "Value is always none")]
pub const _Py_T_NONE: ffi::c_int = 20;

/* Flags */
pub const Py_READONLY: ffi::c_int = 1;
#[cfg(Py_3_10)]
pub const Py_AUDIT_READ: ffi::c_int = 2; // Added in 3.10, harmless no-op before that
#[deprecated]
pub const _Py_WRITE_RESTRICTED: ffi::c_int = 4; // Deprecated, no-op. Do not reuse the value.
pub const Py_RELATIVE_OFFSET: ffi::c_int = 8;

extern "C" {
    pub fn PyMember_GetOne(addr: *const ffi::c_char, l: *mut PyMemberDef) -> *mut PyObject;
    pub fn PyMember_SetOne(
        addr: *mut ffi::c_char,
        l: *mut PyMemberDef,
        value: *mut PyObject,
    ) -> ffi::c_int;
}
