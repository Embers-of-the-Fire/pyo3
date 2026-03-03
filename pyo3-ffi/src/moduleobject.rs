use crate::methodobject::PyMethodDef;
use crate::object::*;
use crate::pyport::Py_ssize_t;
use std::ffi;
use std::ptr::addr_of_mut;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyModule_Type")]
    pub static mut PyModule_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyModule_Check(op: *mut PyObject) -> ffi::c_int {
    PyObject_TypeCheck(op, addr_of_mut!(PyModule_Type))
}

#[inline]
pub unsafe fn PyModule_CheckExact(op: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(op) == addr_of_mut!(PyModule_Type)) as ffi::c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyModule_NewObject")]
    pub fn PyModule_NewObject(name: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyModule_New")]
    pub fn PyModule_New(name: *const ffi::c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyModule_GetDict")]
    pub fn PyModule_GetDict(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg(not(PyPy))]
    pub fn PyModule_GetNameObject(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyModule_GetName")]
    pub fn PyModule_GetName(arg1: *mut PyObject) -> *const ffi::c_char;
    #[cfg(not(all(windows, PyPy)))]
    #[deprecated(note = "Python 3.2")]
    pub fn PyModule_GetFilename(arg1: *mut PyObject) -> *const ffi::c_char;
    #[cfg(not(PyPy))]
    pub fn PyModule_GetFilenameObject(arg1: *mut PyObject) -> *mut PyObject;
    // skipped non-limited _PyModule_Clear
    // skipped non-limited _PyModule_ClearDict
    // skipped non-limited _PyModuleSpec_IsInitializing
    #[cfg_attr(PyPy, link_name = "PyPyModule_GetDef")]
    pub fn PyModule_GetDef(arg1: *mut PyObject) -> *mut PyModuleDef;
    #[cfg_attr(PyPy, link_name = "PyPyModule_GetState")]
    pub fn PyModule_GetState(arg1: *mut PyObject) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyModuleDef_Init")]
    pub fn PyModuleDef_Init(arg1: *mut PyModuleDef) -> *mut PyObject;
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyModuleDef_Type: PyTypeObject;
}

#[repr(C)]
pub struct PyModuleDef_Base {
    pub ob_base: PyObject,
    // Rust function pointers are non-null so an Option is needed here.
    pub m_init: Option<extern "C" fn() -> *mut PyObject>,
    pub m_index: Py_ssize_t,
    pub m_copy: *mut PyObject,
}

#[allow(
    clippy::declare_interior_mutable_const,
    reason = "contains atomic refcount on free-threaded builds"
)]
pub const PyModuleDef_HEAD_INIT: PyModuleDef_Base = PyModuleDef_Base {
    ob_base: PyObject_HEAD_INIT,
    m_init: None,
    m_index: 0,
    m_copy: std::ptr::null_mut(),
};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PyModuleDef_Slot {
    pub slot: ffi::c_int,
    pub value: *mut ffi::c_void,
}

impl Default for PyModuleDef_Slot {
    fn default() -> PyModuleDef_Slot {
        PyModuleDef_Slot {
            slot: 0,
            value: std::ptr::null_mut(),
        }
    }
}

pub const Py_mod_create: ffi::c_int = 1;
pub const Py_mod_exec: ffi::c_int = 2;
#[cfg(Py_3_12)]
pub const Py_mod_multiple_interpreters: ffi::c_int = 3;
#[cfg(Py_3_13)]
pub const Py_mod_gil: ffi::c_int = 4;
#[cfg(Py_3_15)]
pub const Py_mod_abi: ffi::c_int = 5;
#[cfg(Py_3_15)]
pub const Py_mod_name: ffi::c_int = 6;
#[cfg(Py_3_15)]
pub const Py_mod_doc: ffi::c_int = 7;
#[cfg(Py_3_15)]
pub const Py_mod_state_size: ffi::c_int = 8;
#[cfg(Py_3_15)]
pub const Py_mod_methods: ffi::c_int = 9;
#[cfg(Py_3_15)]
pub const Py_mod_state_traverse: ffi::c_int = 10;
#[cfg(Py_3_15)]
pub const Py_mod_state_clear: ffi::c_int = 11;
#[cfg(Py_3_15)]
pub const Py_mod_state_free: ffi::c_int = 12;
#[cfg(Py_3_15)]
pub const Py_mod_token: ffi::c_int = 13;

// skipped private _Py_mod_LAST_SLOT

#[cfg(Py_3_12)]
#[allow(
    clippy::zero_ptr,
    reason = "matches the way that the rest of these constants are defined"
)]
pub const Py_MOD_MULTIPLE_INTERPRETERS_NOT_SUPPORTED: *mut ffi::c_void = 0 as *mut ffi::c_void;
#[cfg(Py_3_12)]
pub const Py_MOD_MULTIPLE_INTERPRETERS_SUPPORTED: *mut ffi::c_void = 1 as *mut ffi::c_void;
#[cfg(Py_3_12)]
pub const Py_MOD_PER_INTERPRETER_GIL_SUPPORTED: *mut ffi::c_void = 2 as *mut ffi::c_void;

#[cfg(Py_3_13)]
#[allow(
    clippy::zero_ptr,
    reason = "matches the way that the rest of these constants are defined"
)]
pub const Py_MOD_GIL_USED: *mut ffi::c_void = 0 as *mut ffi::c_void;
#[cfg(Py_3_13)]
pub const Py_MOD_GIL_NOT_USED: *mut ffi::c_void = 1 as *mut ffi::c_void;

#[cfg(all(not(Py_LIMITED_API), Py_GIL_DISABLED))]
extern "C" {
    pub fn PyUnstable_Module_SetGIL(module: *mut PyObject, gil: *mut ffi::c_void) -> ffi::c_int;
}

#[cfg(Py_3_15)]
extern "C" {
    pub fn PyModule_FromSlotsAndSpec(
        slots: *const PyModuleDef_Slot,
        spec: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyModule_Exec(_mod: *mut PyObject) -> ffi::c_int;
    pub fn PyModule_GetStateSize(_mod: *mut PyObject, result: *mut Py_ssize_t) -> ffi::c_int;
    pub fn PyModule_GetToken(module: *mut PyObject, result: *mut *mut ffi::c_void) -> ffi::c_int;
}

#[repr(C)]
pub struct PyModuleDef {
    pub m_base: PyModuleDef_Base,
    pub m_name: *const ffi::c_char,
    pub m_doc: *const ffi::c_char,
    pub m_size: Py_ssize_t,
    pub m_methods: *mut PyMethodDef,
    pub m_slots: *mut PyModuleDef_Slot,
    // Rust function pointers are non-null so an Option is needed here.
    pub m_traverse: Option<traverseproc>,
    pub m_clear: Option<inquiry>,
    pub m_free: Option<freefunc>,
}
