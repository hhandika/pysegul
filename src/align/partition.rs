use std::path::PathBuf;

use pyo3::{exceptions::PyValueError, prelude::*};
use segul_core::{
    core::align::partition::PartitionConverter,
    helper::{
        partition::construct_partition_path,
        types::{DataType, PartitionFmt},
    },
};

use crate::common::{parse, SEQ_DATA_TYPE_ERR};

use super::PARTITION_FMT_ERR;

/// Convert partition files to another format.
///
/// input_fmt: Input partition format. 'charset', 'nexus', or 'raxml'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_dir: Output directory.
/// output_fmt: Output partition format. Add '-codon' for codon partitions.
/// check_partition: Check that the partitions are valid.
#[pyclass]
pub(crate) struct PartitionConversion {
    input_fmt: PartitionFmt,
    datatype: DataType,
    output_dir: PathBuf,
    output_format: PartitionFmt,
    check_partition: bool,
}

#[pymethods]
impl PartitionConversion {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_dir, output_fmt, check_partition=true))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        output_fmt: &str,
        check_partition: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            input_fmt: parse(input_fmt, PARTITION_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_dir: PathBuf::from(output_dir),
            output_format: parse(output_fmt, PARTITION_FMT_ERR)?,
            check_partition,
        })
    }

    /// Convert the given partition files.
    fn from_files(&self, input_files: Vec<String>) -> PyResult<()> {
        for input in input_files.iter().map(PathBuf::from) {
            let file_name = input.file_name().ok_or_else(|| {
                PyValueError::new_err(format!("Invalid file name: {}", input.display()))
            })?;
            let output_path =
                construct_partition_path(&self.output_dir.join(file_name), &self.output_format);
            PartitionConverter::new(&input, &self.input_fmt, &output_path, &self.output_format)
                .convert(&self.datatype, !self.check_partition);
        }
        Ok(())
    }
}
