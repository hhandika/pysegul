mod concat;
mod convert;
mod filter;
mod split;
mod summary;

use pyo3::prelude::*;

use concat::AlignmentConcatenation;
use convert::AlignmentConversion;
use filter::AlignmentFiltering;
use split::AlignmentSplitting;
use summary::AlignmentSummary;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlignmentConcatenation>()?;
    m.add_class::<AlignmentConversion>()?;
    m.add_class::<AlignmentSummary>()?;
    m.add_class::<AlignmentFiltering>()?;
    m.add_class::<AlignmentSplitting>()?;
    Ok(())
}

const PARTITION_FMT_ERR: &str = "Invalid partition format. \
    Valid options: 'charset', 'charset-codon',\
    'nexus' 'nexus-codon', \
    'raxml', 'raxml-codon'";
