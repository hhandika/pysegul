mod contig;
mod read;

use pyo3::prelude::*;

use crate::genomics::contig::ContigSummary;
use crate::genomics::read::ReadSummary;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReadSummary>()?;
    m.add_class::<ContigSummary>()?;
    Ok(())
}
