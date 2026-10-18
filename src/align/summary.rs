use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::summarize::AlignmentSummary as SeqStats,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

#[pyclass]
pub(crate) struct AlignmentSummary {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    summary_interval: usize,
    output_prefix: Option<String>,
    per_locus: bool,
}

#[pymethods]
impl AlignmentSummary {
    // per_locus: Write a summary for each alignment
    // instead of a summary across all alignments.
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, summary_interval, output_prefix=None, per_locus=false))]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        summary_interval: usize,
        output_prefix: Option<String>,
        per_locus: bool,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<InputFmt>().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            summary_interval,
            output_prefix,
            per_locus,
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
        if self.per_locus {
            handle.summarize_locus(&self.input_files, self.output_prefix.as_deref());
        } else {
            handle.summarize_all(&self.input_files, self.output_prefix.as_deref());
        }
    }
}
