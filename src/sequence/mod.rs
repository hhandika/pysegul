mod extract;
mod id;
mod remove;
mod translate;

use extract::SequenceExtraction;
use id::IDExtraction;
use pyo3::prelude::*;
use remove::SequenceRemoval;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IDExtraction>()?;
    m.add_class::<SequenceExtraction>()?;
    m.add_class::<SequenceRemoval>()?;
    Ok(())
}
