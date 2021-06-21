use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
#[text_signature = "(v)"]
/// Say hello.
fn hello(v: Vec<i32>) -> PyResult<usize> {
    for x in v.iter() {
        println!("Hello, {}", x);
    }
    Ok(v.len())
}

#[pymodule]
/// Sample module.
fn maturin_sample(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}
