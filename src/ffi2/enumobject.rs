use ffi2::object::PyTypeObject;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyEnum_Type: PyTypeObject;
    pub static mut PyReversed_Type: PyTypeObject;
}
