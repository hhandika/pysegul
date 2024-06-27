use std::path::PathBuf;

use pyo3::prelude::*;
use segul::{
    handler::align::partition::PartConverter,
    helper::{
        partition::construct_partition_path,
        types::{DataType, PartitionFmt},
    },
};

use crate::common::SEQ_DATA_TYPE_ERR;

use super::PARTITION_FMT_ERR;

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
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        output_fmt: &str,
        check_partition: bool,
    ) -> Self {
        Self {
            input_fmt: input_fmt.parse::<PartitionFmt>().expect(PARTITION_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(SEQ_DATA_TYPE_ERR),
            output_dir: PathBuf::from(output_dir),
            output_format: output_fmt.parse::<PartitionFmt>().expect(PARTITION_FMT_ERR),
            check_partition,
        }
    }

    pub(crate) fn from_files(&self, input_files: Vec<String>) {
        input_files.iter().map(PathBuf::from).for_each(|f| {
            let output_path = self
                .output_dir
                .join(f.file_name().expect("Invalid file name"));
            let final_path = construct_partition_path(&output_path, &self.output_format);
            self.convert_partitions(f, final_path)
        })
    }

    fn convert_partitions(&self, input_path: PathBuf, output_path: PathBuf) {
        let handle = PartConverter::new(
            &input_path,
            &self.input_fmt,
            &output_path,
            &self.output_format,
        );
        handle.convert(&self.datatype, self.check_partition)
    }
}
