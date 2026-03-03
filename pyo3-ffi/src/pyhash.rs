#[cfg(not(any(Py_LIMITED_API, PyPy)))]
use crate::pyport::{Py_hash_t, Py_ssize_t};
use std::ffi;

extern "C" {
    // skipped non-limited _Py_HashDouble
    // skipped non-limited _Py_HashPointer
    // skipped non-limited _Py_HashPointerRaw

    #[cfg(not(any(Py_LIMITED_API, PyPy)))]
    pub fn _Py_HashBytes(src: *const ffi::c_void, len: Py_ssize_t) -> Py_hash_t;
}

pub const _PyHASH_MULTIPLIER: ffi::c_ulong = 1000003;

// skipped _PyHASH_BITS

// skipped non-limited _Py_HashSecret_t

// skipped Py_HASH_CUTOFF

pub const Py_HASH_EXTERNAL: ffi::c_int = 0;
pub const Py_HASH_SIPHASH24: ffi::c_int = 1;
pub const Py_HASH_FNV: ffi::c_int = 2;

// skipped Py_HASH_ALGORITHM
