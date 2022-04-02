pub mod interpreter;
pub mod operators;
pub mod value;

use crate::interpreter::Interpreter;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

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
