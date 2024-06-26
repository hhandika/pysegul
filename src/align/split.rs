use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::align::split,
    helper::types::{DataType, InputFmt, OutputFmt, PartitionFmt},
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

use super::PARTITION_FMT_ERR;

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
    pub(crate) fn new(
        input_path: &str,
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        output_fmt: &str,
        partition_fmt: &str,
        check_partition: bool,
        input_partition: Option<String>,
        output_prefix: Option<String>,
    ) -> Self {
        Self {
            input_path: PathBuf::from(input_path),
            input_fmt: input_fmt.parse::<InputFmt>().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(SEQ_DATA_TYPE_ERR),
            output_dir: PathBuf::from(output_dir),
            output_fmt: output_fmt.parse::<OutputFmt>().expect(PARTITION_FMT_ERR),
            partition_fmt: partition_fmt
                .parse::<PartitionFmt>()
                .expect(PARTITION_FMT_ERR),
            check_partition,
            output_prefix,
            input_partition: input_partition.map(PathBuf::from),
        }
    }

    fn split(&mut self) {
        let input_partition = match &self.input_partition {
            Some(partition) => partition,
            // Assume it is charset partition
            // in the same file as the input alignment
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
            &input_partition,
            &self.partition_fmt,
            &self.output_prefix,
            self.check_partition,
        );
    }
}
