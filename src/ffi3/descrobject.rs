use ffi3::methodobject::PyMethodDef;
use ffi3::object::{PyObject, PyTypeObject};
use ffi3::structmember::PyMemberDef;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub type getter = unsafe extern "C" fn(slf: *mut PyObject, closure: *mut c_void) -> *mut PyObject;

pub type setter =
    unsafe extern "C" fn(slf: *mut PyObject, value: *mut PyObject, closure: *mut c_void) -> c_int;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PyGetSetDef {
    pub name: *mut c_char,
    pub get: Option<getter>,
    pub set: Option<setter>,
    pub doc: *mut c_char,
    pub closure: *mut c_void,
}

pub const PyGetSetDef_INIT: PyGetSetDef = PyGetSetDef {
    name: ptr::null_mut(),
    get: None,
    set: None,
    doc: ptr::null_mut(),
    closure: ptr::null_mut(),
};

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyClassMethodDescr_Type: PyTypeObject;
    pub static mut PyGetSetDescr_Type: PyTypeObject;
    pub static mut PyMemberDescr_Type: PyTypeObject;
    pub static mut PyMethodDescr_Type: PyTypeObject;
    pub static mut PyWrapperDescr_Type: PyTypeObject;
    pub static mut PyDictProxy_Type: PyTypeObject;

    pub fn PyDescr_NewMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef) -> *mut PyObject;
    pub fn PyDescr_NewClassMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef)
        -> *mut PyObject;
    pub fn PyDescr_NewMember(arg1: *mut PyTypeObject, arg2: *mut PyMemberDef) -> *mut PyObject;
    pub fn PyDescr_NewGetSet(arg1: *mut PyTypeObject, arg2: *mut PyGetSetDef) -> *mut PyObject;
    pub fn PyDictProxy_New(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyWrapper_New(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;

    pub static mut PyProperty_Type: PyTypeObject;
}
