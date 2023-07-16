
#![feature(specialization)]

#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(sum_as_string))?;

    Ok(())
}
