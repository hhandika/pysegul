//! Remove sequences across multiple files or a directory.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::remove::{SeqRemovalParameters as RemoveOpts, SequenceRemoval as Remove},
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{parse, to_paths, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! remove_seq {
    ($self: ident, $input_files: ident, $params: ident) => {
        let handle = Remove::new(
            &$self.input_fmt,
            &$self.datatype,
            &$self.output_path,
            &$self.output_format,
            &$params,
        );
        handle.remove(&$self.$input_files)
    };
}

/// Remove sequences by ID or regular expression. Set the input with
/// `input_files` or `input_dir`, then call a removal method.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_format: 'fasta', 'nexus', 'phylip', or their interleaved forms.
#[pyclass]
pub(crate) struct SequenceRemoval {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: OutputFmt,
}

#[pymethods]
impl SequenceRemoval {
    #[new]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_format: &str,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_format: parse(output_format, SEQ_OUTPUT_FMT_ERR)?,
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

    /// Remove sequences whose IDs match a regular expression.
    fn remove_regex(&self, regex: String) {
        let parameters = RemoveOpts::Regex(regex);
        remove_seq!(self, input_files, parameters);
    }

    /// Remove sequences with the given IDs.
    fn remove_id_list(&self, id: Vec<String>) {
        let parameters = RemoveOpts::Id(id);
        remove_seq!(self, input_files, parameters);
    }
}
