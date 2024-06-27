use std::path::PathBuf;

use pyo3::prelude::*;
use segul::{
    handler::align::partition::PartConverter,
    helper::types::{DataType, PartitionFmt},
};

use crate::common::SEQ_DATA_TYPE_ERR;

use super::PARTITION_FMT_ERR;

#[pyclass]
pub(crate) struct PartitionConversion {
    input_fmt: PartitionFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: PartitionFmt,
    check_partition: bool,
}

#[pymethods]
impl PartitionConversion {
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        check_partition: bool,
    ) -> Self {
        Self {
            input_fmt: input_fmt.parse::<PartitionFmt>().expect(PARTITION_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_format: output_fmt.parse::<PartitionFmt>().expect(PARTITION_FMT_ERR),
            check_partition,
        }
    }

    pub(crate) fn from_files(&self, input_files: Vec<String>) {
        input_files
            .iter()
            .map(PathBuf::from)
            .for_each(|f| self.convert_partitions(f))
    }

    fn convert_partitions(&self, input_path: PathBuf) {
        let handle = PartConverter::new(
            &input_path,
            &self.input_fmt,
            &self.output_path,
            &self.output_format,
        );
        handle.convert(&self.datatype, self.check_partition)
    }
}
