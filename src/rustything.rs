use pyo3::prelude::*;

#[pyfunction]
fn rust_function(a: &str) -> PyResult<usize> {
    Ok("Hello, world! " + a)
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn _rustything(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_function, m)?)?;

    Ok(())
}
