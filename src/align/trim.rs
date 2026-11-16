//! Trim alignment sites across many alignments at once.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::trim::{AlignmentTrimming as Trimming, TrimmingParameters},
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR, parse, to_paths};

/// Trim alignment sites. Set the input with `input_files` or `input_dir`,
/// then call a trimming method.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
#[pyclass]
pub(crate) struct AlignmentTrimming {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
}

#[pymethods]
impl AlignmentTrimming {
    #[new]
    fn new(input_fmt: &str, datatype: &str, output_path: &str, output_fmt: &str) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
        })
    }

    /// Input alignment files.
    #[setter(input_files)]
    fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
    }

    /// Directory of input alignment files.
    #[setter(input_dir)]
    fn set_input_dir(&mut self, input_dir: &str) {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
    }

    /// Remove sites whose proportion of missing data (gaps and '?')
    /// is above `max_missing` (0.0 - 1.0).
    fn missing_data(&self, max_missing: f64) {
        self.trim(TrimmingParameters::MissingData(max_missing));
    }

    /// Trim based on a threshold of parsimony informative sites.
    fn parsimony_inf(&self, threshold: usize) {
        self.trim(TrimmingParameters::ParsInf(threshold));
    }
}

impl AlignmentTrimming {
    fn trim(&self, params: TrimmingParameters) {
        Trimming::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &self.output_fmt,
            &params,
        )
        .trim();
    }
}
