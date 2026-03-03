use libc::size_t;
use std::ffi;

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyMem_Malloc")]
    pub fn PyMem_Malloc(size: size_t) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyMem_Calloc")]
    pub fn PyMem_Calloc(nelem: size_t, elsize: size_t) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyMem_Realloc")]
    pub fn PyMem_Realloc(ptr: *mut ffi::c_void, new_size: size_t) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyMem_Free")]
    pub fn PyMem_Free(ptr: *mut ffi::c_void);
}
