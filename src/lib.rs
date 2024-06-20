mod align;
mod genomics;

use pyo3::prelude::*;

#[pymodule]
fn pysegul(m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    genomics::register(m)?;
    Ok(())
}
