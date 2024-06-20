use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::read::summarize::ReadSummaryHandler,
    helper::{
        finder::SeqReadFinder,
        types::{SeqReadFmt, SummaryMode},
    },
};

const INPUT_FMT_ERR: &str = "Invalid input format. Valid options: 'auto', 'fastq', 'gzip'";
const SUMMARY_MODE_ERR: &str =
    "Invalid summary mode. Valid options: 'default', 'minimal', 'complete'";

#[pyclass]
pub(crate) struct ReadSummary {
    input_files: Vec<PathBuf>,
    input_fmt: SeqReadFmt,
    mode: SummaryMode,
    output_path: PathBuf,
    output_prefix: Option<String>,
}

#[pymethods]
impl ReadSummary {
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        mode: &str,
        output_path: &str,
        output_prefix: Option<String>,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<SeqReadFmt>().expect(INPUT_FMT_ERR),
            mode: mode.parse().expect(SUMMARY_MODE_ERR),
            output_path: PathBuf::from(output_path),
            output_prefix,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.summarize_reads();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqReadFinder::new(input_dir).find(&self.input_fmt);
        self.summarize_reads();
    }

    fn summarize_reads(&mut self) {
        let handle = ReadSummaryHandler::new(
            &mut self.input_files,
            &self.input_fmt,
            &self.mode,
            &self.output_path,
            self.output_prefix.as_deref(),
        );
        handle.summarize();
    }
}
