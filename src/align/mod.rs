mod concat;
mod convert;
mod filter;
mod split;
mod summary;

use pyo3::prelude::*;

use crate::align::concat::AlignmentConcatenation;
use crate::align::convert::convert_alignments;
use crate::align::filter::AlignmentFiltering;
use crate::align::summary::summarize_alignments;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlignmentConcatenation>()?;
    m.add_function(wrap_pyfunction!(summarize_alignments, m)?)?;
    m.add_function(wrap_pyfunction!(convert_alignments, m)?)?;
    m.add_class::<AlignmentFiltering>()?;
    Ok(())
}
