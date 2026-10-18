mod contig;
mod maf;
mod read;

use pyo3::prelude::*;

use crate::genomics::contig::ContigSummary;
use crate::genomics::maf::{MafConversion, MafSummary};
use crate::genomics::read::ReadSummary;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReadSummary>()?;
    m.add_class::<ContigSummary>()?;
    m.add_class::<MafConversion>()?;
    m.add_class::<MafSummary>()?;
    Ok(())
}
