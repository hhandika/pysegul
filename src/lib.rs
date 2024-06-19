mod align;

use pyo3::prelude::*;

#[pymodule]
fn pysegul(m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    Ok(())
}
