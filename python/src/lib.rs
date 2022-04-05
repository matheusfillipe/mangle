/// Python library for mangle
/// This provides bindings for evaluating mangle code from python

use mangle::interpreter::Interpreter;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Evaluates string as mangle code
#[pyfunction]
fn eval(line: String) -> PyResult<String> {
    let mut interpreter = Interpreter::new(' ');
    match interpreter.eval_line(&line) {
        Ok(result) => Ok(result),
        Err(err) => Err(PyValueError::new_err(format!("{}", err))),
    }
}

/// Mangle module python bindings
#[pymodule]
fn mangle(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}
