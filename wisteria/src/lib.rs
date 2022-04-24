use pyo3::prelude::*;

#[pyfunction]
fn wisteria1(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn wisteria(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wisteria1, m)?)?;
    Ok(())
}