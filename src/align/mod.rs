mod concat;

use pyo3::prelude::*;

use crate::align::concat::concat_alignments;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat_alignments, m)?)?;
    Ok(())
}
