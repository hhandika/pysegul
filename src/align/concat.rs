use std::path::{Path, PathBuf};

use pyo3::prelude::*;

use segul::{
    handler::align::concat::ConcatHandler,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

use super::{DATA_TYPE_ERR, INPUT_FMT_ERR, OUTPUT_FMT_ERR, PARTITION_FMT_ERR};

#[pyclass]
pub struct AlignmentConcatenation {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_prefix: String,
    output_fmt: OutputFmt,
    partition_fmt: PartitionFmt,
}

#[pymethods]
impl AlignmentConcatenation {
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_prefix: &str,
        output_fmt: &str,
        partition_fmt: &str,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<InputFmt>().expect(INPUT_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(DATA_TYPE_ERR),
            output_prefix: output_prefix.to_string(),
            output_fmt: output_fmt.parse::<OutputFmt>().expect(OUTPUT_FMT_ERR),
            partition_fmt: partition_fmt
                .parse::<PartitionFmt>()
                .expect(PARTITION_FMT_ERR),
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.concat_alignments();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.concat_alignments();
    }

    fn concat_alignments(&mut self) {
        let output = Path::new(&self.output_prefix);
        let mut handle = ConcatHandler::new(
            &self.input_fmt,
            output,
            &self.output_fmt,
            &self.partition_fmt,
        );
        handle.concat_alignment(&mut self.input_files, &self.datatype);
    }
}
