use std::collections::HashMap;
use pyo3::prelude::*;



#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    let mut hm = HashMap::new();
    for sub_str in s.split_whitespace() {
        let count = hm
            .entry(sub_str)
            .or_insert(0);
        *count += 1;
    }
    
    return Python::with_gil(|py| {
        hm.to_object(py)
    });
}

#[pyfunction]
fn calculate_pi(n_terms: usize) -> PyResult<f64> {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
