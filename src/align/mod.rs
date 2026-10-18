mod concat;
mod convert;
mod distance;
mod filter;
mod partition;
mod split;
mod summary;
mod trim;
mod unalign;

use partition::PartitionConversion;
use pyo3::prelude::*;

use concat::AlignmentConcatenation;
use convert::AlignmentConversion;
use distance::AlignmentDistance;
use filter::AlignmentFiltering;
use split::AlignmentSplitting;
use summary::AlignmentSummary;
use trim::AlignmentTrimming;
use unalign::AlignmentUnalignment;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlignmentConcatenation>()?;
    m.add_class::<AlignmentConversion>()?;
    m.add_class::<AlignmentSummary>()?;
    m.add_class::<AlignmentFiltering>()?;
    m.add_class::<AlignmentSplitting>()?;
    m.add_class::<PartitionConversion>()?;
    m.add_class::<AlignmentDistance>()?;
    m.add_class::<AlignmentTrimming>()?;
    m.add_class::<AlignmentUnalignment>()?;
    Ok(())
}

const PARTITION_FMT_ERR: &str = "Invalid partition format. \
    Valid options: 'charset', 'charset-codon',\
    'nexus' 'nexus-codon', \
    'raxml', 'raxml-codon'";
