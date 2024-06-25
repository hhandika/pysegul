mod align;
mod genomics;
mod utils;

use pyo3::prelude::*;

#[pymodule]
fn pysegul(m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    genomics::register(m)?;
    utils::register(m)?;

    Ok(())
}
