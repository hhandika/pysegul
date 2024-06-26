//! Remove sequences across multiple files or a directory.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::sequence::remove::{Remove, RemoveOpts},
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

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

    pub(crate) fn remove_regex(&self, regex: String) {
        let parameters = RemoveOpts::Regex(regex);
        remove_seq!(self, input_files, parameters);
    }

    pub(crate) fn remove_id_list(&self, id: Vec<String>) {
        let parameters = RemoveOpts::Id(id);
        remove_seq!(self, input_files, parameters);
    }
}
