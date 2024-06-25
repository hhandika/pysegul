mod align;
mod common;
mod genomics;
mod sequence;
mod utils;

use pyo3::prelude::*;

#[pymodule]
fn pysegul(m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    genomics::register(m)?;
    sequence::register(m)?;
    utils::register(m)?;

    Ok(())
}
