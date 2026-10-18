//! Filter sequences across many sequence files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::filter::{SeqFilteringParameters, SequenceFiltering as Filtering},
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! filter_seq {
    ($self: ident, $params: ident) => {
        let handle = Filtering::new(
            &$self.input_files,
            &$self.input_fmt,
            &$self.datatype,
            &$self.output_path,
            &$self.output_fmt,
            &$params,
        );
        handle.filter();
    };
}

#[pyclass]
pub(crate) struct SequenceFiltering {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
}

#[pymethods]
impl SequenceFiltering {
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

    // Remove sequences shorter than `min_length` (gaps excluded).
    pub(crate) fn minimal_length(&self, min_length: usize) {
        let params = SeqFilteringParameters::MinSequenceLength(min_length);
        filter_seq!(self, params);
    }

    // Remove sequences longer than `max_length` (gaps excluded).
    pub(crate) fn maximal_length(&self, max_length: usize) {
        let params = SeqFilteringParameters::MaxSequenceLength(max_length);
        filter_seq!(self, params);
    }

    // Remove sequences whose proportion of gaps is above `max_gap` (0.0 - 1.0).
    pub(crate) fn maximal_gap(&self, max_gap: f64) {
        let params = SeqFilteringParameters::PercentMaxGap(max_gap);
        filter_seq!(self, params);
    }
}
