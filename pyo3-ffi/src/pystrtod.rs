use crate::object::PyObject;
use std::ffi;

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyOS_string_to_double")]
    pub fn PyOS_string_to_double(
        str: *const ffi::c_char,
        endptr: *mut *mut ffi::c_char,
        overflow_exception: *mut PyObject,
    ) -> ffi::c_double;
    #[cfg_attr(PyPy, link_name = "PyPyOS_double_to_string")]
    pub fn PyOS_double_to_string(
        val: ffi::c_double,
        format_code: ffi::c_char,
        precision: ffi::c_int,
        flags: ffi::c_int,
        _type: *mut ffi::c_int,
    ) -> *mut ffi::c_char;
}

// skipped non-limited _Py_string_to_number_with_underscores
// skipped non-limited _Py_parse_inf_or_nan

/* PyOS_double_to_string's "flags" parameter can be set to 0 or more of: */
pub const Py_DTSF_SIGN: ffi::c_int = 0x01; /* always add the sign */
pub const Py_DTSF_ADD_DOT_0: ffi::c_int = 0x02; /* if the result is an integer add ".0" */
pub const Py_DTSF_ALT: ffi::c_int = 0x04; /* "alternate" formatting. it's format_code specific */

/* PyOS_double_to_string's "type", if non-NULL, will be set to one of: */
pub const Py_DTST_FINITE: ffi::c_int = 0;
pub const Py_DTST_INFINITE: ffi::c_int = 1;
pub const Py_DTST_NAN: ffi::c_int = 2;
