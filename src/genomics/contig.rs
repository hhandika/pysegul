use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::contig::summarize::ContigSummaryHandler,
    helper::{finder::ContigFileFinder, types::ContigFmt},
};

const INPUT_FMT_ERR: &str = "Invalid input format. Valid options: 'auto', 'fasta', 'gzip'";

#[pyclass]
pub(crate) struct ContigSummary {
    input_files: Vec<PathBuf>,
    input_fmt: ContigFmt,
    output_dir: PathBuf,
    output_prefix: Option<String>,
}

#[pymethods]
impl ContigSummary {
    #[new]
    pub(crate) fn new(input_fmt: &str, output_dir: &str, output_prefix: Option<String>) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<ContigFmt>().expect(INPUT_FMT_ERR),
            output_dir: PathBuf::from(output_dir),
            output_prefix,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.summarize_contigs();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = ContigFileFinder::new(input_dir).find(&self.input_fmt);
        self.summarize_contigs();
    }

    fn summarize_contigs(&mut self) {
        let handle = ContigSummaryHandler::new(
            &mut self.input_files,
            &self.input_fmt,
            &self.output_dir,
            self.output_prefix.as_deref(),
        );
        handle.summarize();
    }
}
