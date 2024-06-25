mod id;

use id::IDExtraction;
use pyo3::prelude::*;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IDExtraction>()?;
    Ok(())
}
