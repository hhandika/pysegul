mod extract;
mod id;

use extract::SequenceExtraction;
use id::IDExtraction;
use pyo3::prelude::*;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IDExtraction>()?;
    m.add_class::<SequenceExtraction>()?;
    Ok(())
}
