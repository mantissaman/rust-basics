#[macro_use]
extern crate pyo3;
use pyo3::prelude::*;

/// This module is a python module implemented in Rust.
#[pymodinit]
fn word_suffix(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_function!(find_words))?;
    Ok(())
}

#[pyfunction]
fn find_words(src: &str, suffix: &str) -> PyResult<Vec<String>> {
    let mut v = vec![];
    let filtered = src.split(",").filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.ends_with(&suffix) {
            Some(trimmed.to_owned())
        } else {
            None
        }
    });

    for s in filtered {
        v.push(s);
    }
    Ok(v)
}
// cargo install maturing
// python3 -m venv venv
// 