use crate::{PyConfig, PyPreConfig, PyStatus, Py_ssize_t};
use libc::wchar_t;
use std::ffi;

extern "C" {

    // skipped Py_FrozenMain

    pub fn Py_PreInitialize(src_config: *const PyPreConfig) -> PyStatus;
    pub fn Py_PreInitializeFromBytesArgs(
        src_config: *const PyPreConfig,
        argc: Py_ssize_t,
        argv: *mut *mut ffi::c_char,
    ) -> PyStatus;
    pub fn Py_PreInitializeFromArgs(
        src_config: *const PyPreConfig,
        argc: Py_ssize_t,
        argv: *mut *mut wchar_t,
    ) -> PyStatus;

    pub fn Py_InitializeFromConfig(config: *const PyConfig) -> PyStatus;

    pub fn Py_RunMain() -> ffi::c_int;

    pub fn Py_ExitStatusException(status: PyStatus) -> !;

    // skipped Py_FdIsInteractive
}

#[cfg(Py_3_12)]
pub const PyInterpreterConfig_DEFAULT_GIL: ffi::c_int = 0;
#[cfg(Py_3_12)]
pub const PyInterpreterConfig_SHARED_GIL: ffi::c_int = 1;
#[cfg(Py_3_12)]
pub const PyInterpreterConfig_OWN_GIL: ffi::c_int = 2;

#[cfg(Py_3_12)]
#[repr(C)]
pub struct PyInterpreterConfig {
    pub use_main_obmalloc: ffi::c_int,
    pub allow_fork: ffi::c_int,
    pub allow_exec: ffi::c_int,
    pub allow_threads: ffi::c_int,
    pub allow_daemon_threads: ffi::c_int,
    pub check_multi_interp_extensions: ffi::c_int,
    pub gil: ffi::c_int,
}

#[cfg(Py_3_12)]
pub const _PyInterpreterConfig_INIT: PyInterpreterConfig = PyInterpreterConfig {
    use_main_obmalloc: 0,
    allow_fork: 0,
    allow_exec: 0,
    allow_threads: 1,
    allow_daemon_threads: 0,
    check_multi_interp_extensions: 1,
    gil: PyInterpreterConfig_OWN_GIL,
};

// https://github.com/python/cpython/blob/902de283a8303177eb95bf5bc252d2421fcbd758/Include/cpython/pylifecycle.h#L63-L65
#[cfg(Py_3_12)]
const _PyInterpreterConfig_LEGACY_CHECK_MULTI_INTERP_EXTENSIONS: ffi::c_int =
    if cfg!(Py_GIL_DISABLED) { 1 } else { 0 };

#[cfg(Py_3_12)]
pub const _PyInterpreterConfig_LEGACY_INIT: PyInterpreterConfig = PyInterpreterConfig {
    use_main_obmalloc: 1,
    allow_fork: 1,
    allow_exec: 1,
    allow_threads: 1,
    allow_daemon_threads: 1,
    check_multi_interp_extensions: _PyInterpreterConfig_LEGACY_CHECK_MULTI_INTERP_EXTENSIONS,
    gil: PyInterpreterConfig_SHARED_GIL,
};

extern "C" {
    #[cfg(Py_3_12)]
    pub fn Py_NewInterpreterFromConfig(
        tstate_p: *mut *mut crate::PyThreadState,
        config: *const PyInterpreterConfig,
    ) -> PyStatus;
}

// skipped atexit_datacallbackfunc
// skipped PyUnstable_AtExit
