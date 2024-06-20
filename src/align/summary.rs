use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::align::summarize::SeqStats,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
};

#[pyclass]
pub struct AlignmentSummarization {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_prefix: String,
    summary_interval: usize,
}

#[pymethods]
impl AlignmentSummarization {
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        summary_interval: usize,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<InputFmt>().expect("Invalid input format"),
            datatype: datatype.parse::<DataType>().expect("Invalid data type"),
            output_path: PathBuf::from(output_path),
            output_prefix: String::new(),
            summary_interval,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.summarize_alignments();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.summarize_alignments();
    }

    fn summarize_alignments(&mut self) {
        let mut handle = SeqStats::new(
            &self.input_fmt,
            &self.output_path,
            self.summary_interval,
            &self.datatype,
        );
        handle.summarize_all(&mut self.input_files, Some(&self.output_prefix));
    }
}
