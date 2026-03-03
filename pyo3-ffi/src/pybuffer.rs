use crate::object::PyObject;
use crate::pyport::Py_ssize_t;
use std::ffi;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Py_buffer {
    pub buf: *mut ffi::c_void,
    /// Owned reference
    pub obj: *mut crate::PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: ffi::c_int,
    pub ndim: ffi::c_int,
    pub format: *mut ffi::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub internal: *mut ffi::c_void,
    #[cfg(PyPy)]
    pub flags: ffi::c_int,
    #[cfg(PyPy)]
    pub _strides: [Py_ssize_t; PyBUF_MAX_NDIM],
    #[cfg(PyPy)]
    pub _shape: [Py_ssize_t; PyBUF_MAX_NDIM],
}

impl Py_buffer {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Py_buffer {
            buf: ptr::null_mut(),
            obj: ptr::null_mut(),
            len: 0,
            itemsize: 0,
            readonly: 0,
            ndim: 0,
            format: ptr::null_mut(),
            shape: ptr::null_mut(),
            strides: ptr::null_mut(),
            suboffsets: ptr::null_mut(),
            internal: ptr::null_mut(),
            #[cfg(PyPy)]
            flags: 0,
            #[cfg(PyPy)]
            _strides: [0; PyBUF_MAX_NDIM],
            #[cfg(PyPy)]
            _shape: [0; PyBUF_MAX_NDIM],
        }
    }
}

pub type getbufferproc =
    unsafe extern "C" fn(*mut PyObject, *mut crate::Py_buffer, ffi::c_int) -> ffi::c_int;
pub type releasebufferproc = unsafe extern "C" fn(*mut PyObject, *mut crate::Py_buffer);

/* Return 1 if the getbuffer function is available, otherwise return 0. */
extern "C" {
    #[cfg(not(PyPy))]
    pub fn PyObject_CheckBuffer(obj: *mut PyObject) -> ffi::c_int;

    #[cfg_attr(PyPy, link_name = "PyPyObject_GetBuffer")]
    pub fn PyObject_GetBuffer(
        obj: *mut PyObject,
        view: *mut Py_buffer,
        flags: ffi::c_int,
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_GetPointer")]
    pub fn PyBuffer_GetPointer(
        view: *const Py_buffer,
        indices: *const Py_ssize_t,
    ) -> *mut ffi::c_void;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_SizeFromFormat")]
    pub fn PyBuffer_SizeFromFormat(format: *const ffi::c_char) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_ToContiguous")]
    pub fn PyBuffer_ToContiguous(
        buf: *mut ffi::c_void,
        view: *const Py_buffer,
        len: Py_ssize_t,
        order: ffi::c_char,
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FromContiguous")]
    pub fn PyBuffer_FromContiguous(
        view: *const Py_buffer,
        buf: *const ffi::c_void,
        len: Py_ssize_t,
        order: ffi::c_char,
    ) -> ffi::c_int;
    pub fn PyObject_CopyData(dest: *mut PyObject, src: *mut PyObject) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_IsContiguous")]
    pub fn PyBuffer_IsContiguous(view: *const Py_buffer, fort: ffi::c_char) -> ffi::c_int;
    pub fn PyBuffer_FillContiguousStrides(
        ndims: ffi::c_int,
        shape: *mut Py_ssize_t,
        strides: *mut Py_ssize_t,
        itemsize: ffi::c_int,
        fort: ffi::c_char,
    );
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FillInfo")]
    pub fn PyBuffer_FillInfo(
        view: *mut Py_buffer,
        o: *mut PyObject,
        buf: *mut ffi::c_void,
        len: Py_ssize_t,
        readonly: ffi::c_int,
        flags: ffi::c_int,
    ) -> ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_Release")]
    pub fn PyBuffer_Release(view: *mut Py_buffer);
}

/// Maximum number of dimensions
pub const PyBUF_MAX_NDIM: usize = 64;

/* Flags for getting buffers */
pub const PyBUF_SIMPLE: ffi::c_int = 0;
pub const PyBUF_WRITABLE: ffi::c_int = 0x0001;
/* we used to include an E, backwards compatible alias */
pub const PyBUF_WRITEABLE: ffi::c_int = PyBUF_WRITABLE;
pub const PyBUF_FORMAT: ffi::c_int = 0x0004;
pub const PyBUF_ND: ffi::c_int = 0x0008;
pub const PyBUF_STRIDES: ffi::c_int = 0x0010 | PyBUF_ND;
pub const PyBUF_C_CONTIGUOUS: ffi::c_int = 0x0020 | PyBUF_STRIDES;
pub const PyBUF_F_CONTIGUOUS: ffi::c_int = 0x0040 | PyBUF_STRIDES;
pub const PyBUF_ANY_CONTIGUOUS: ffi::c_int = 0x0080 | PyBUF_STRIDES;
pub const PyBUF_INDIRECT: ffi::c_int = 0x0100 | PyBUF_STRIDES;

pub const PyBUF_CONTIG: ffi::c_int = PyBUF_ND | PyBUF_WRITABLE;
pub const PyBUF_CONTIG_RO: ffi::c_int = PyBUF_ND;

pub const PyBUF_STRIDED: ffi::c_int = PyBUF_STRIDES | PyBUF_WRITABLE;
pub const PyBUF_STRIDED_RO: ffi::c_int = PyBUF_STRIDES;

pub const PyBUF_RECORDS: ffi::c_int = PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT;
pub const PyBUF_RECORDS_RO: ffi::c_int = PyBUF_STRIDES | PyBUF_FORMAT;

pub const PyBUF_FULL: ffi::c_int = PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT;
pub const PyBUF_FULL_RO: ffi::c_int = PyBUF_INDIRECT | PyBUF_FORMAT;

pub const PyBUF_READ: ffi::c_int = 0x100;
pub const PyBUF_WRITE: ffi::c_int = 0x200;
