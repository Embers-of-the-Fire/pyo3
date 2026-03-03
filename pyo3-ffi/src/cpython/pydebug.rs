use std::ffi;

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_DebugFlag")]
    pub static mut Py_DebugFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_VerboseFlag")]
    pub static mut Py_VerboseFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_QuietFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_InteractiveFlag")]
    pub static mut Py_InteractiveFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_InspectFlag")]
    pub static mut Py_InspectFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_OptimizeFlag")]
    pub static mut Py_OptimizeFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_NoSiteFlag")]
    pub static mut Py_NoSiteFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_BytesWarningFlag")]
    pub static mut Py_BytesWarningFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_UseClassExceptionsFlag")]
    pub static mut Py_UseClassExceptionsFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_FrozenFlag")]
    pub static mut Py_FrozenFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_IgnoreEnvironmentFlag")]
    pub static mut Py_IgnoreEnvironmentFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_DontWriteBytecodeFlag")]
    pub static mut Py_DontWriteBytecodeFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    #[cfg_attr(PyPy, link_name = "PyPy_NoUserSiteDirectory")]
    pub static mut Py_NoUserSiteDirectory: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_UnbufferedStdioFlag: ffi::c_int;
    #[cfg_attr(PyPy, link_name = "PyPy_HashRandomizationFlag")]
    pub static mut Py_HashRandomizationFlag: ffi::c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_IsolatedFlag: ffi::c_int;
    #[cfg(windows)]
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_LegacyWindowsFSEncodingFlag: ffi::c_int;
    #[cfg(windows)]
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_LegacyWindowsStdioFlag: ffi::c_int;
}

extern "C" {
    #[cfg(Py_3_11)]
    pub fn Py_GETENV(name: *const ffi::c_char) -> *mut ffi::c_char;
}

#[cfg(not(Py_3_11))]
#[inline(always)]
pub unsafe fn Py_GETENV(name: *const ffi::c_char) -> *mut ffi::c_char {
    #[allow(deprecated)]
    if Py_IgnoreEnvironmentFlag != 0 {
        std::ptr::null_mut()
    } else {
        libc::getenv(name)
    }
}
