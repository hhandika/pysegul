mod concat;
mod convert;
mod filter;
mod split;
mod summary;

use pyo3::prelude::*;

use crate::align::concat::AlignmentConcatenation;
use crate::align::convert::AlignmentConversion;
use crate::align::filter::AlignmentFiltering;
use crate::align::summary::AlignmentSummarization;

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlignmentConcatenation>()?;
    m.add_class::<AlignmentConversion>()?;
    m.add_class::<AlignmentSummarization>()?;
    m.add_class::<AlignmentFiltering>()?;
    Ok(())
}

const INPUT_FMT_ERR: &str = "Invalid input format. Valid options: 'fasta', 'nexus', 'phylip'";
const DATA_TYPE_ERR: &str = "Invalid data type. Valid options: 'dna', 'aa', 'ignore'";
const PARTITION_FMT_ERR: &str = "Invalid partition format. \
    Valid options: 'charset', 'charset-codon',\
    'nexus' 'nexus-codon', \
    'raxml', 'raxml-codon'";
const OUTPUT_FMT_ERR: &str = "Invalid output format. \
    Valid options: 'fasta', \
    'nexus', 'phylip',\
    'fast-int', 'nexus-int', 'phylip-int'";
