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

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR, parse, to_paths};

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

/// Filter sequences. Set the input with `input_files` or `input_dir`,
/// then call a filtering method.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
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
    fn new(input_fmt: &str, datatype: &str, output_path: &str, output_fmt: &str) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
        })
    }

    /// Input sequence files.
    #[setter(input_files)]
    fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
    }

    /// Directory of input sequence files.
    #[setter(input_dir)]
    fn set_input_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
    }

    /// Remove sequences shorter than `min_length` (gaps excluded).
    fn minimal_length(&self, min_length: usize) {
        let params = SeqFilteringParameters::MinSequenceLength(min_length);
        filter_seq!(self, params);
    }

    /// Remove sequences longer than `max_length` (gaps excluded).
    fn maximal_length(&self, max_length: usize) {
        let params = SeqFilteringParameters::MaxSequenceLength(max_length);
        filter_seq!(self, params);
    }

    /// Remove sequences whose proportion of gaps is above `max_gap` (0.0 - 1.0).
    fn maximal_gap(&self, max_gap: f64) {
        let params = SeqFilteringParameters::PercentMaxGap(max_gap);
        filter_seq!(self, params);
    }
}
