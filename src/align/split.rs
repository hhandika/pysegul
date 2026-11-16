use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::split,
    helper::types::{DataType, InputFmt, OutputFmt, PartitionFmt},
};

use crate::common::{parse, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

use super::PARTITION_FMT_ERR;

/// Split a concatenated alignment into one alignment per partition.
///
/// input_path: Concatenated alignment file.
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_dir: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
/// partition_fmt: Partition format. 'charset', 'nexus', or 'raxml'.
/// check_partition: Check that the partitions are valid.
/// input_partition: Partition file. Defaults to the charset block in the input file.
/// output_prefix: Prefix of the output file names.
#[pyclass]
pub(crate) struct AlignmentSplitting {
    input_path: PathBuf,
    input_fmt: InputFmt,
    datatype: DataType,
    output_dir: PathBuf,
    output_fmt: OutputFmt,
    partition_fmt: PartitionFmt,
    check_partition: bool,
    output_prefix: Option<String>,
    input_partition: Option<PathBuf>,
}

#[pymethods]
impl AlignmentSplitting {
    #[new]
    #[pyo3(signature = (
        input_path,
        input_fmt,
        datatype,
        output_dir,
        output_fmt,
        partition_fmt,
        check_partition=true,
        input_partition=None,
        output_prefix=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        input_path: &str,
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        output_fmt: &str,
        partition_fmt: &str,
        check_partition: bool,
        input_partition: Option<String>,
        output_prefix: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_path: PathBuf::from(input_path),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_dir: PathBuf::from(output_dir),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
            partition_fmt: parse(partition_fmt, PARTITION_FMT_ERR)?,
            check_partition,
            output_prefix,
            input_partition: input_partition.map(PathBuf::from),
        })
    }

    /// Split the alignment.
    fn split(&self) {
        // Without a partition file, read the charset block of the alignment.
        let input_partition = match &self.input_partition {
            Some(partition) => partition,
            None => Path::new(&self.input_path),
        };
        let handle = split::AlignmentSplitting::new(
            &self.input_path,
            &self.datatype,
            &self.input_fmt,
            &self.output_dir,
            &self.output_fmt,
        );
        handle.split(
            input_partition,
            &self.partition_fmt,
            &self.output_prefix,
            !self.check_partition,
        );
    }
}
