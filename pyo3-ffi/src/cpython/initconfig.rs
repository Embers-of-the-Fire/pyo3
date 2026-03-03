/* --- PyStatus ----------------------------------------------- */

use crate::Py_ssize_t;
use libc::wchar_t;
use std::ffi;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum _PyStatus_TYPE {
    _PyStatus_TYPE_OK = 0,
    _PyStatus_TYPE_ERROR = 1,
    _PyStatus_TYPE_EXIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyStatus {
    pub _type: _PyStatus_TYPE,
    pub func: *const ffi::c_char,
    pub err_msg: *const ffi::c_char,
    pub exitcode: ffi::c_int,
}

extern "C" {
    pub fn PyStatus_Ok() -> PyStatus;
    pub fn PyStatus_Error(err_msg: *const ffi::c_char) -> PyStatus;
    pub fn PyStatus_NoMemory() -> PyStatus;
    pub fn PyStatus_Exit(exitcode: ffi::c_int) -> PyStatus;
    pub fn PyStatus_IsError(err: PyStatus) -> ffi::c_int;
    pub fn PyStatus_IsExit(err: PyStatus) -> ffi::c_int;
    pub fn PyStatus_Exception(err: PyStatus) -> ffi::c_int;
}

/* --- PyWideStringList ------------------------------------------------ */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyWideStringList {
    pub length: Py_ssize_t,
    pub items: *mut *mut wchar_t,
}

extern "C" {
    pub fn PyWideStringList_Append(list: *mut PyWideStringList, item: *const wchar_t) -> PyStatus;
    pub fn PyWideStringList_Insert(
        list: *mut PyWideStringList,
        index: Py_ssize_t,
        item: *const wchar_t,
    ) -> PyStatus;
}

/* --- PyPreConfig ----------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyPreConfig {
    pub _config_init: ffi::c_int,
    pub parse_argv: ffi::c_int,
    pub isolated: ffi::c_int,
    pub use_environment: ffi::c_int,
    pub configure_locale: ffi::c_int,
    pub coerce_c_locale: ffi::c_int,
    pub coerce_c_locale_warn: ffi::c_int,

    #[cfg(windows)]
    pub legacy_windows_fs_encoding: ffi::c_int,

    pub utf8_mode: ffi::c_int,
    pub dev_mode: ffi::c_int,
    pub allocator: ffi::c_int,
}

extern "C" {
    pub fn PyPreConfig_InitPythonConfig(config: *mut PyPreConfig);
    pub fn PyPreConfig_InitIsolatedConfig(config: *mut PyPreConfig);
}

/* --- PyConfig ---------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyConfig {
    pub _config_init: ffi::c_int,
    pub isolated: ffi::c_int,
    pub use_environment: ffi::c_int,
    pub dev_mode: ffi::c_int,
    pub install_signal_handlers: ffi::c_int,
    pub use_hash_seed: ffi::c_int,
    pub hash_seed: ffi::c_ulong,
    pub faulthandler: ffi::c_int,
    #[cfg(all(Py_3_9, not(Py_3_10)))]
    pub _use_peg_parser: ffi::c_int,
    pub tracemalloc: ffi::c_int,
    #[cfg(Py_3_12)]
    pub perf_profiling: ffi::c_int,
    #[cfg(Py_3_14)]
    pub remote_debug: ffi::c_int,
    pub import_time: ffi::c_int,
    #[cfg(Py_3_11)]
    pub code_debug_ranges: ffi::c_int,
    pub show_ref_count: ffi::c_int,
    #[cfg(not(Py_3_9))]
    pub show_alloc_count: ffi::c_int,
    pub dump_refs: ffi::c_int,
    #[cfg(Py_3_11)]
    pub dump_refs_file: *mut wchar_t,
    pub malloc_stats: ffi::c_int,
    pub filesystem_encoding: *mut wchar_t,
    pub filesystem_errors: *mut wchar_t,
    pub pycache_prefix: *mut wchar_t,
    pub parse_argv: ffi::c_int,
    #[cfg(Py_3_10)]
    pub orig_argv: PyWideStringList,
    pub argv: PyWideStringList,
    #[cfg(not(Py_3_10))]
    pub program_name: *mut wchar_t,
    pub xoptions: PyWideStringList,
    pub warnoptions: PyWideStringList,
    pub site_import: ffi::c_int,
    pub bytes_warning: ffi::c_int,
    #[cfg(Py_3_10)]
    pub warn_default_encoding: ffi::c_int,
    pub inspect: ffi::c_int,
    pub interactive: ffi::c_int,
    pub optimization_level: ffi::c_int,
    pub parser_debug: ffi::c_int,
    pub write_bytecode: ffi::c_int,
    pub verbose: ffi::c_int,
    pub quiet: ffi::c_int,
    pub user_site_directory: ffi::c_int,
    pub configure_c_stdio: ffi::c_int,
    pub buffered_stdio: ffi::c_int,
    pub stdio_encoding: *mut wchar_t,
    pub stdio_errors: *mut wchar_t,

    #[cfg(windows)]
    pub legacy_windows_stdio: ffi::c_int,

    pub check_hash_pycs_mode: *mut wchar_t,
    #[cfg(Py_3_11)]
    pub use_frozen_modules: ffi::c_int,
    #[cfg(Py_3_11)]
    pub safe_path: ffi::c_int,
    #[cfg(Py_3_12)]
    pub int_max_str_digits: ffi::c_int,
    #[cfg(Py_3_14)]
    pub thread_inherit_context: ffi::c_int,
    #[cfg(Py_3_14)]
    pub context_aware_warnings: ffi::c_int,
    #[cfg(all(Py_3_14, target_os = "macos"))]
    pub use_system_logger: ffi::c_int,
    #[cfg(Py_3_13)]
    pub cpu_count: ffi::c_int,
    #[cfg(Py_GIL_DISABLED)]
    pub enable_gil: ffi::c_int,
    #[cfg(all(Py_3_14, Py_GIL_DISABLED))]
    pub tlbc_enabled: ffi::c_int,
    pub pathconfig_warnings: ffi::c_int,
    #[cfg(Py_3_10)]
    pub program_name: *mut wchar_t,
    pub pythonpath_env: *mut wchar_t,
    pub home: *mut wchar_t,
    #[cfg(Py_3_10)]
    pub platlibdir: *mut wchar_t,

    pub module_search_paths_set: ffi::c_int,
    pub module_search_paths: PyWideStringList,
    #[cfg(Py_3_11)]
    pub stdlib_dir: *mut wchar_t,
    pub executable: *mut wchar_t,
    pub base_executable: *mut wchar_t,
    pub prefix: *mut wchar_t,
    pub base_prefix: *mut wchar_t,
    pub exec_prefix: *mut wchar_t,
    pub base_exec_prefix: *mut wchar_t,
    #[cfg(all(Py_3_9, not(Py_3_10)))]
    pub platlibdir: *mut wchar_t,
    pub skip_source_first_line: ffi::c_int,
    pub run_command: *mut wchar_t,
    pub run_module: *mut wchar_t,
    pub run_filename: *mut wchar_t,
    #[cfg(Py_3_13)]
    pub sys_path_0: *mut wchar_t,
    pub _install_importlib: ffi::c_int,
    pub _init_main: ffi::c_int,
    #[cfg(all(Py_3_9, not(Py_3_12)))]
    pub _isolated_interpreter: ffi::c_int,
    #[cfg(Py_3_11)]
    pub _is_python_build: ffi::c_int,
    #[cfg(all(Py_3_9, not(Py_3_10)))]
    pub _orig_argv: PyWideStringList,
    #[cfg(all(Py_3_13, py_sys_config = "Py_DEBUG"))]
    pub run_presite: *mut wchar_t,
}

extern "C" {
    pub fn PyConfig_InitPythonConfig(config: *mut PyConfig);
    pub fn PyConfig_InitIsolatedConfig(config: *mut PyConfig);
    pub fn PyConfig_Clear(config: *mut PyConfig);
    pub fn PyConfig_SetString(
        config: *mut PyConfig,
        config_str: *mut *mut wchar_t,
        str: *const wchar_t,
    ) -> PyStatus;
    pub fn PyConfig_SetBytesString(
        config: *mut PyConfig,
        config_str: *mut *mut wchar_t,
        str: *const ffi::c_char,
    ) -> PyStatus;
    pub fn PyConfig_Read(config: *mut PyConfig) -> PyStatus;
    pub fn PyConfig_SetBytesArgv(
        config: *mut PyConfig,
        argc: Py_ssize_t,
        argv: *mut *const ffi::c_char,
    ) -> PyStatus;
    pub fn PyConfig_SetArgv(
        config: *mut PyConfig,
        argc: Py_ssize_t,
        argv: *mut *const wchar_t,
    ) -> PyStatus;
    pub fn PyConfig_SetWideStringList(
        config: *mut PyConfig,
        list: *mut PyWideStringList,
        length: Py_ssize_t,
        items: *mut *mut wchar_t,
    ) -> PyStatus;
}

/* --- Helper functions --------------------------------------- */

extern "C" {
    pub fn Py_GetArgcArgv(argc: *mut ffi::c_int, argv: *mut *mut *mut wchar_t);
}
