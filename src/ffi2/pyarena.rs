use ffi2::object::PyObject;
use libc::size_t;
use std::os::raw::{c_int, c_void};

#[allow(missing_copy_implementations)]
pub enum PyArena {
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyArena_New() -> *mut PyArena;
    pub fn PyArena_Free(arg1: *mut PyArena);
    pub fn PyArena_Malloc(arg1: *mut PyArena, size: size_t) -> *mut c_void;
    pub fn PyArena_AddPyObject(arg1: *mut PyArena, arg2: *mut PyObject) -> c_int;
}
