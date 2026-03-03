use crate::object::PyObject;
use std::ffi;

extern "C" {
    pub fn PyImport_GetMagicNumber() -> ffi::c_long;
    pub fn PyImport_GetMagicTag() -> *const ffi::c_char;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ExecCodeModule")]
    pub fn PyImport_ExecCodeModule(name: *const ffi::c_char, co: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ExecCodeModuleEx")]
    pub fn PyImport_ExecCodeModuleEx(
        name: *const ffi::c_char,
        co: *mut PyObject,
        pathname: *const ffi::c_char,
    ) -> *mut PyObject;
    pub fn PyImport_ExecCodeModuleWithPathnames(
        name: *const ffi::c_char,
        co: *mut PyObject,
        pathname: *const ffi::c_char,
        cpathname: *const ffi::c_char,
    ) -> *mut PyObject;
    pub fn PyImport_ExecCodeModuleObject(
        name: *mut PyObject,
        co: *mut PyObject,
        pathname: *mut PyObject,
        cpathname: *mut PyObject,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_GetModuleDict")]
    pub fn PyImport_GetModuleDict() -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_GetModule")]
    pub fn PyImport_GetModule(name: *mut PyObject) -> *mut PyObject;
    pub fn PyImport_AddModuleObject(name: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_AddModule")]
    pub fn PyImport_AddModule(name: *const ffi::c_char) -> *mut PyObject;
    #[cfg(Py_3_13)]
    #[cfg_attr(PyPy, link_name = "PyPyImport_AddModuleRef")]
    pub fn PyImport_AddModuleRef(name: *const ffi::c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModule")]
    pub fn PyImport_ImportModule(name: *const ffi::c_char) -> *mut PyObject;
    #[deprecated(note = "Python 3.13")]
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleNoBlock")]
    pub fn PyImport_ImportModuleNoBlock(name: *const ffi::c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleLevel")]
    pub fn PyImport_ImportModuleLevel(
        name: *const ffi::c_char,
        globals: *mut PyObject,
        locals: *mut PyObject,
        fromlist: *mut PyObject,
        level: ffi::c_int,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleLevelObject")]
    pub fn PyImport_ImportModuleLevelObject(
        name: *mut PyObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
        fromlist: *mut PyObject,
        level: ffi::c_int,
    ) -> *mut PyObject;
}

#[inline]
pub unsafe fn PyImport_ImportModuleEx(
    name: *const ffi::c_char,
    globals: *mut PyObject,
    locals: *mut PyObject,
    fromlist: *mut PyObject,
) -> *mut PyObject {
    PyImport_ImportModuleLevel(name, globals, locals, fromlist, 0)
}

extern "C" {
    pub fn PyImport_GetImporter(path: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_Import")]
    pub fn PyImport_Import(name: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ReloadModule")]
    pub fn PyImport_ReloadModule(m: *mut PyObject) -> *mut PyObject;
    #[cfg(not(Py_3_9))]
    #[deprecated(note = "Removed in Python 3.9 as it was \"For internal use only\".")]
    pub fn PyImport_Cleanup();
    pub fn PyImport_ImportFrozenModuleObject(name: *mut PyObject) -> ffi::c_int;
    pub fn PyImport_ImportFrozenModule(name: *const ffi::c_char) -> ffi::c_int;

    pub fn PyImport_AppendInittab(
        name: *const ffi::c_char,
        initfunc: Option<unsafe extern "C" fn() -> *mut PyObject>,
    ) -> ffi::c_int;
}
