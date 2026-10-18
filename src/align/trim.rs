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

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! trim_aln {
    ($self: ident, $params: ident) => {
        let handle = Trimming::new(
            &$self.input_files,
            &$self.input_fmt,
            &$self.datatype,
            &$self.output_path,
            &$self.output_fmt,
            &$params,
        );
        handle.trim();
    };
}

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
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_fmt: output_fmt.parse().expect(SEQ_OUTPUT_FMT_ERR),
        }
    }

    // Input from files
    #[setter(input_files)]
    pub(crate) fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
    }

    // Input from a directory
    #[setter(input_dir)]
    pub(crate) fn set_input_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
    }

    // Remove sites whose proportion of missing data (gaps and '?')
    // is above `max_missing` (0.0 - 1.0).
    pub(crate) fn missing_data(&self, max_missing: f64) {
        let params = TrimmingParameters::MissingData(max_missing);
        trim_aln!(self, params);
    }

    // Trim based on a threshold of parsimony informative sites.
    pub(crate) fn parsimony_inf(&self, threshold: usize) {
        let params = TrimmingParameters::ParsInf(threshold);
        trim_aln!(self, params);
    }
}
