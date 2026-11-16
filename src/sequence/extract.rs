//! Extract sequences from many sequence files.

use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyFileNotFoundError, prelude::*};
use segul_core::core::sequence::extract::{
    SeqExtractionParameters as ExtractOpts, SequenceExtraction as Extract,
};
use segul_core::helper::types::{DataType, OutputFmt};
use segul_core::helper::{finder::SeqFileFinder, types::InputFmt};
use segul_core::parser::txt;

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR, parse, to_paths};

macro_rules! extract_seq {
    ($self: ident, $input_files: ident, $parameters: ident) => {
        let handle = Extract::new(
            &$self.input_fmt,
            &$self.datatype,
            &$parameters,
            &$self.output_path,
            &$self.output_format,
        );
        handle.extract(&$self.$input_files)
    };
}

/// Extract sequences by ID or regular expression. Set the input with
/// `input_files` or `input_dir`, then call an extraction method.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_format: 'fasta', 'nexus', 'phylip', or their interleaved forms.
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

    /// Extract sequences whose IDs match a regular expression (Rust regex syntax).
    fn extract_regex(&self, regex: String) {
        let params = ExtractOpts::Regex(regex);
        extract_seq!(self, input_files, params);
    }

    /// Extract sequences with the given IDs.
    fn extract_id_list(&self, list: Vec<String>) {
        let params = ExtractOpts::Id(list);
        extract_seq!(self, input_files, params);
    }

    /// Extract sequences with the IDs listed in a text file, one per line.
    fn extract_id_file(&self, path: &str) -> PyResult<()> {
        let path = Path::new(path);
        if !path.is_file() {
            return Err(PyFileNotFoundError::new_err(format!(
                "File does not exist: {}",
                path.display()
            )));
        }
        let params = ExtractOpts::Id(txt::parse_text_file(path));
        extract_seq!(self, input_files, params);
        Ok(())
    }
}
