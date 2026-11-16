// `from_files` and `from_dir` run the task, so they take `self`.
#![allow(clippy::wrong_self_convention)]

mod align;
mod common;
mod genomics;
mod plot;
mod report;
mod sequence;
mod utils;

use pyo3::prelude::*;

#[pymodule]
fn pysegul(m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    genomics::register(m)?;
    sequence::register(m)?;
    plot::register(m)?;
    report::register(m)?;
    utils::register(m)?;

    Ok(())
}
