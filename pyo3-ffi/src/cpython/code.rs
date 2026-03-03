use crate::object::*;
use crate::pyport::Py_ssize_t;

#[cfg(not(GraalPy))]
use crate::PyCodeObject;
use std::ffi;
#[cfg(not(PyPy))]
use std::ptr::addr_of_mut;

// skipped private _PY_MONITORING_LOCAL_EVENTS
// skipped private _PY_MONITORING_UNGROUPED_EVENTS
// skipped private _PY_MONITORING_EVENTS

// skipped private _PyLocalMonitors
// skipped private _Py_GlobalMonitors

// skipped private _Py_CODEUNIT

// skipped private _Py_OPCODE
// skipped private _Py_OPARG

// skipped private _py_make_codeunit

// skipped private _py_set_opcode

// skipped private _Py_MAKE_CODEUNIT
// skipped private _Py_SET_OPCODE

// skipped private _PyCoCached
// skipped private _PyCoLineInstrumentationData
// skipped private _PyCoMonitoringData

// skipped private _PyExecutorArray

/* Masks for co_flags */
pub const CO_OPTIMIZED: ffi::c_int = 0x0001;
pub const CO_NEWLOCALS: ffi::c_int = 0x0002;
pub const CO_VARARGS: ffi::c_int = 0x0004;
pub const CO_VARKEYWORDS: ffi::c_int = 0x0008;
pub const CO_NESTED: ffi::c_int = 0x0010;
pub const CO_GENERATOR: ffi::c_int = 0x0020;
/* The CO_NOFREE flag is set if there are no free or cell variables.
   This information is redundant, but it allows a single flag test
   to determine whether there is any extra work to be done when the
   call frame it setup.
*/
pub const CO_NOFREE: ffi::c_int = 0x0040;
/* The CO_COROUTINE flag is set for coroutine functions (defined with
``async def`` keywords) */
pub const CO_COROUTINE: ffi::c_int = 0x0080;
pub const CO_ITERABLE_COROUTINE: ffi::c_int = 0x0100;
pub const CO_ASYNC_GENERATOR: ffi::c_int = 0x0200;

pub const CO_FUTURE_DIVISION: ffi::c_int = 0x2000;
pub const CO_FUTURE_ABSOLUTE_IMPORT: ffi::c_int = 0x4000; /* do absolute imports by default */
pub const CO_FUTURE_WITH_STATEMENT: ffi::c_int = 0x8000;
pub const CO_FUTURE_PRINT_FUNCTION: ffi::c_int = 0x1_0000;
pub const CO_FUTURE_UNICODE_LITERALS: ffi::c_int = 0x2_0000;

pub const CO_FUTURE_BARRY_AS_BDFL: ffi::c_int = 0x4_0000;
pub const CO_FUTURE_GENERATOR_STOP: ffi::c_int = 0x8_0000;
// skipped CO_FUTURE_ANNOTATIONS
// skipped CO_CELL_NOT_AN_ARG

pub const CO_MAXBLOCKS: usize = 20;

#[cfg(not(PyPy))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyCode_Type: PyTypeObject;
}

#[inline]
#[cfg(not(PyPy))]
pub unsafe fn PyCode_Check(op: *mut PyObject) -> ffi::c_int {
    (Py_TYPE(op) == addr_of_mut!(PyCode_Type)) as ffi::c_int
}

extern "C" {
    #[cfg(PyPy)]
    #[link_name = "PyPyCode_Check"]
    pub fn PyCode_Check(op: *mut PyObject) -> ffi::c_int;
}

// skipped PyCode_GetNumFree (requires knowledge of code object layout)

extern "C" {
    #[cfg(not(GraalPy))]
    #[cfg_attr(PyPy, link_name = "PyPyCode_New")]
    pub fn PyCode_New(
        argcount: ffi::c_int,
        kwonlyargcount: ffi::c_int,
        nlocals: ffi::c_int,
        stacksize: ffi::c_int,
        flags: ffi::c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        firstlineno: ffi::c_int,
        lnotab: *mut PyObject,
    ) -> *mut PyCodeObject;
    #[cfg(not(GraalPy))]
    #[cfg(Py_3_8)]
    pub fn PyCode_NewWithPosOnlyArgs(
        argcount: ffi::c_int,
        posonlyargcount: ffi::c_int,
        kwonlyargcount: ffi::c_int,
        nlocals: ffi::c_int,
        stacksize: ffi::c_int,
        flags: ffi::c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        firstlineno: ffi::c_int,
        lnotab: *mut PyObject,
    ) -> *mut PyCodeObject;
    #[cfg(not(GraalPy))]
    #[cfg_attr(PyPy, link_name = "PyPyCode_NewEmpty")]
    pub fn PyCode_NewEmpty(
        filename: *const ffi::c_char,
        funcname: *const ffi::c_char,
        firstlineno: ffi::c_int,
    ) -> *mut PyCodeObject;
    #[cfg(not(GraalPy))]
    pub fn PyCode_Addr2Line(arg1: *mut PyCodeObject, arg2: ffi::c_int) -> ffi::c_int;
    // skipped PyCodeAddressRange "for internal use only"
    // skipped _PyCode_CheckLineNumber
    // skipped _PyCode_ConstantKey
    pub fn PyCode_Optimize(
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        lnotab: *mut PyObject,
    ) -> *mut PyObject;
    pub fn _PyCode_GetExtra(
        code: *mut PyObject,
        index: Py_ssize_t,
        extra: *const *mut ffi::c_void,
    ) -> ffi::c_int;
    pub fn _PyCode_SetExtra(
        code: *mut PyObject,
        index: Py_ssize_t,
        extra: *mut ffi::c_void,
    ) -> ffi::c_int;
}
