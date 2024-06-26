//! Extract sequences from many sequence files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::handler::sequence::extract::{Extract, ExtractOpts};
use segul::helper::types::{DataType, OutputFmt};
use segul::helper::{finder::SeqFileFinder, types::InputFmt};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! extract_seq {
    ($self: ident, $input_files: ident, $parameters: ident) => {
        let handle = Extract::new(
            &$self.input_fmt,
            &$self.datatype,
            &$parameters,
            &$self.output_path,
            &$self.output_format,
        );
        handle.extract_sequences(&$self.$input_files)
    };
}

#[pyclass]
pub(crate) struct SequenceExtraction {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: OutputFmt,
}

#[pymethods]
impl SequenceExtraction {
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_format: &str,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_format: output_format.parse().expect(SEQ_OUTPUT_FMT_ERR),
        }
    }

    // Input from files
    pub(crate) fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
    }

    // Input from a directory
    pub(crate) fn set_input_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
    }

    // Extract sequences using a regular expression
    // Follow Rust regex syntax: https://docs.rs/regex/latest/regex/
    fn extract_regex(&self, regex: String) {
        let params = ExtractOpts::Regex(regex);
        extract_seq!(self, input_files, params);
    }
}
