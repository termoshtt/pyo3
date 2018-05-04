#![feature(proc_macro, specialization)]

extern crate pyo3;

use pyo3::prelude::*;

use pyo3::py::class as pyclass;
use pyo3::py::methods as pymethods;

#[macro_use]
mod common;

#[pyclass]
struct InstanceMethod {
    member: i32,
    token: PyToken,
}

#[pymethods]
impl InstanceMethod {
    /// Test method
    fn method(&self) -> PyResult<i32> {
        Ok(self.member)
    }
}

#[test]
fn instance_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = py.init_ref(|t| InstanceMethod {
        member: 42,
        token: t,
    }).unwrap();
    assert!(obj.method().unwrap() == 42);
    let d = PyDict::new(py);
    d.set_item("obj", obj).unwrap();
    py.run("assert obj.method() == 42", None, Some(d)).unwrap();
    py.run("assert obj.method.__doc__ == 'Test method'", None, Some(d))
        .unwrap();
}

#[pyclass]
struct InstanceMethodWithArgs {
    member: i32,
    token: PyToken,
}

#[pymethods]
impl InstanceMethodWithArgs {
    fn method(&self, multiplier: i32) -> PyResult<i32> {
        Ok(self.member * multiplier)
    }
}

//#[test]
fn instance_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = py.init_ref(|t| InstanceMethodWithArgs {
        member: 7,
        token: t,
    }).unwrap();
    assert!(obj.method(6).unwrap() == 42);
    let d = PyDict::new(py);
    d.set_item("obj", obj).unwrap();
    py.run("assert obj.method(3) == 21", None, Some(d)).unwrap();
    py.run("assert obj.method(multiplier=6) == 42", None, Some(d))
        .unwrap();
}

#[pyclass]
struct ClassMethod {
    token: PyToken,
}

#[pymethods]
impl ClassMethod {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|t| ClassMethod { token: t })
    }

    #[classmethod]
    fn method(cls: &PyType) -> PyResult<String> {
        Ok(format!("{}.method()!", cls.name()))
    }
}

#[test]
fn class_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = PyDict::new(py);
    d.set_item("C", py.get_type::<ClassMethod>()).unwrap();
    py.run(
        "assert C.method() == 'ClassMethod.method()!'",
        None,
        Some(d),
    ).unwrap();
    py.run(
        "assert C().method() == 'ClassMethod.method()!'",
        None,
        Some(d),
    ).unwrap();
}

#[pyclass]
struct ClassMethodWithArgs {
    token: PyToken,
}

#[pymethods]
impl ClassMethodWithArgs {
    #[classmethod]
    fn method(cls: &PyType, input: &PyString) -> PyResult<String> {
        Ok(format!("{}.method({})", cls.name(), input))
    }
}

#[test]
fn class_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = PyDict::new(py);
    d.set_item("C", py.get_type::<ClassMethodWithArgs>())
        .unwrap();
    py.run(
        "assert C.method('abc') == 'ClassMethodWithArgs.method(abc)'",
        None,
        Some(d),
    ).unwrap();
}

#[pyclass]
struct StaticMethod {
    token: PyToken,
}

#[pymethods]
impl StaticMethod {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|t| StaticMethod { token: t })
    }

    #[staticmethod]
    fn method(_py: Python) -> PyResult<&'static str> {
        Ok("StaticMethod.method()!")
    }
}

#[test]
fn static_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethod::method(py).unwrap(), "StaticMethod.method()!");
    let d = PyDict::new(py);
    d.set_item("C", py.get_type::<StaticMethod>()).unwrap();
    py.run(
        "assert C.method() == 'StaticMethod.method()!'",
        None,
        Some(d),
    ).unwrap();
    py.run(
        "assert C().method() == 'StaticMethod.method()!'",
        None,
        Some(d),
    ).unwrap();
}

#[pyclass]
struct StaticMethodWithArgs {
    token: PyToken,
}

#[pymethods]
impl StaticMethodWithArgs {
    #[staticmethod]
    fn method(_py: Python, input: i32) -> PyResult<String> {
        Ok(format!("0x{:x}", input))
    }
}

#[test]
fn static_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethodWithArgs::method(py, 1234).unwrap(), "0x4d2");

    let d = PyDict::new(py);
    d.set_item("C", py.get_type::<StaticMethodWithArgs>())
        .unwrap();
    py.run("assert C.method(1337) == '0x539'", None, Some(d))
        .unwrap();
}

#[pyclass]
struct MethArgs {
    token: PyToken,
}

#[pymethods]
impl MethArgs {
    #[args(test = "10")]
    fn get_default(&self, test: i32) -> PyResult<i32> {
        Ok(test)
    }
    #[args("*", test = 10)]
    fn get_kwarg(&self, test: i32) -> PyResult<i32> {
        Ok(test)
    }
    #[args(args = "*", kwargs = "**")]
    fn get_kwargs(&self, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
        Ok([args.into(), kwargs.to_object(self.py())].to_object(self.py()))
    }
}

#[test]
fn meth_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let inst = py.init(|t| MethArgs { token: t }).unwrap();

    py_run!(py, inst, "assert inst.get_default() == 10");
    py_run!(py, inst, "assert inst.get_default(100) == 100");
    py_run!(py, inst, "assert inst.get_kwarg() == 10");
    py_run!(py, inst, "assert inst.get_kwarg(100) == 10");
    py_run!(py, inst, "assert inst.get_kwarg(test=100) == 100");
    py_run!(py, inst, "assert inst.get_kwargs() == [(), None]");
    py_run!(py, inst, "assert inst.get_kwargs(1,2,3) == [(1,2,3), None]");
    py_run!(
        py,
        inst,
        "assert inst.get_kwargs(t=1,n=2) == [(), {'t': 1, 'n': 2}]"
    );
    py_run!(
        py,
        inst,
        "assert inst.get_kwargs(1,2,3,t=1,n=2) == [(1,2,3), {'t': 1, 'n': 2}]"
    );
    // py_expect_exception!(py, inst, "inst.get_kwarg(100)", TypeError);
}
