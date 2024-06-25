mod concat;
mod convert;
mod filter;
mod split;
mod summary;

use pyo3::prelude::*;

use crate::align::concat::AlignmentConcatenation;
use crate::align::convert::AlignmentConversion;
use crate::align::filter::AlignmentFiltering;
use crate::align::summary::AlignmentSummary;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlignmentConcatenation>()?;
    m.add_class::<AlignmentConversion>()?;
    m.add_class::<AlignmentSummary>()?;
    m.add_class::<AlignmentFiltering>()?;
    Ok(())
}

const PARTITION_FMT_ERR: &str = "Invalid partition format. \
    Valid options: 'charset', 'charset-codon',\
    'nexus' 'nexus-codon', \
    'raxml', 'raxml-codon'";
