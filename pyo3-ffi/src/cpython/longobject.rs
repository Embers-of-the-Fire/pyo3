use crate::longobject::*;
use crate::object::*;
#[cfg(Py_3_13)]
use crate::pyport::Py_ssize_t;
use libc::size_t;
use std::ffi;

#[cfg(Py_3_13)]
extern "C" {
    pub fn PyLong_FromUnicodeObject(u: *mut PyObject, base: ffi::c_int) -> *mut PyObject;
}

#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_DEFAULTS: ffi::c_int = -1;
#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_BIG_ENDIAN: ffi::c_int = 0;
#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_LITTLE_ENDIAN: ffi::c_int = 1;
#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_NATIVE_ENDIAN: ffi::c_int = 3;
#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_UNSIGNED_BUFFER: ffi::c_int = 4;
#[cfg(Py_3_13)]
pub const Py_ASNATIVEBYTES_REJECT_NEGATIVE: ffi::c_int = 8;

extern "C" {
    // skipped _PyLong_Sign

    #[cfg(Py_3_13)]
    pub fn PyLong_AsNativeBytes(
        v: *mut PyObject,
        buffer: *mut ffi::c_void,
        n_bytes: Py_ssize_t,
        flags: ffi::c_int,
    ) -> Py_ssize_t;

    #[cfg(Py_3_13)]
    pub fn PyLong_FromNativeBytes(
        buffer: *const ffi::c_void,
        n_bytes: size_t,
        flags: ffi::c_int,
    ) -> *mut PyObject;

    #[cfg(Py_3_13)]
    pub fn PyLong_FromUnsignedNativeBytes(
        buffer: *const ffi::c_void,
        n_bytes: size_t,
        flags: ffi::c_int,
    ) -> *mut PyObject;

    // skipped PyUnstable_Long_IsCompact
    // skipped PyUnstable_Long_CompactValue

    #[cfg_attr(PyPy, link_name = "_PyPyLong_FromByteArray")]
    pub fn _PyLong_FromByteArray(
        bytes: *const ffi::c_uchar,
        n: size_t,
        little_endian: ffi::c_int,
        is_signed: ffi::c_int,
    ) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "_PyPyLong_AsByteArrayO")]
    pub fn _PyLong_AsByteArray(
        v: *mut PyLongObject,
        bytes: *mut ffi::c_uchar,
        n: size_t,
        little_endian: ffi::c_int,
        is_signed: ffi::c_int,
    ) -> ffi::c_int;

    // skipped _PyLong_GCD
}
