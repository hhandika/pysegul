use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::align::split::AlignmentSplitting,
    helper::types::{DataType, InputFmt, OutputFmt, PartitionFmt},
};

use super::{DATA_TYPE_ERR, INPUT_FMT_ERR, PARTITION_FMT_ERR};

#[pyclass]
pub(crate) struct AlignmentSplit {
    input_path: PathBuf,
    input_fmt: InputFmt,
    datatype: DataType,
    output_dir: PathBuf,
    output_fmt: OutputFmt,
    partition_fmt: PartitionFmt,
    uncheck_partition: bool,
    output_prefix: Option<String>,
    input_partition: Option<PathBuf>,
}

#[pymethods]
impl AlignmentSplit {
    #[new]
    pub(crate) fn new(
        input_path: &str,
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        partition_fmt: &str,
        uncheck_partition: bool,
        output_prefix: Option<String>,
        input_partition: Option<String>,
    ) -> Self {
        Self {
            input_path: PathBuf::from(input_path),
            input_fmt: input_fmt.parse::<InputFmt>().expect(INPUT_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(DATA_TYPE_ERR),
            output_dir: PathBuf::from(output_dir),
            output_fmt: partition_fmt.parse::<OutputFmt>().expect(PARTITION_FMT_ERR),
            partition_fmt: partition_fmt
                .parse::<PartitionFmt>()
                .expect(PARTITION_FMT_ERR),
            uncheck_partition,
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
        let handle = AlignmentSplitting::new(
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
            self.uncheck_partition,
        );
    }
}
