//! Rename sequence IDs on many alignments at once.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::rename::{
        SeqRenamingParameters, SequenceRenaming as Renaming, SequenceRenamingDry,
    },
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
    parser::delimited,
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! rename_seq {
    ($self: ident, $params: ident) => {
        if $self.dry_run {
            SequenceRenamingDry::new(&$self.input_fmt, &$self.datatype, &$params)
                .dry_run(&$self.input_files);
        } else {
            Renaming::new(
                &$self.input_fmt,
                &$self.datatype,
                &$self.output_path,
                &$self.output_format,
                &$params,
            )
            .rename(&$self.input_files);
        }
    };
}

#[pyclass]
pub(crate) struct SequenceRenaming {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: OutputFmt,
    dry_run: bool,
}

#[pymethods]
impl SequenceRenaming {
    // dry_run: Log the renaming results without writing any files.
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_format, dry_run=false))]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_format: &str,
        dry_run: bool,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_format: output_format.parse().expect(SEQ_OUTPUT_FMT_ERR),
            dry_run,
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

    // Replace IDs using a list of (original ID, new ID) pairs.
    pub(crate) fn replace_id(&self, names: Vec<(String, String)>) {
        let params = SeqRenamingParameters::RnId(names);
        rename_seq!(self, params);
    }

    // Replace IDs using a CSV or TSV file with a header row
    // and two columns: original ID and new ID.
    pub(crate) fn replace_id_from_file(&self, path: &str) {
        let names = delimited::parse_delimited_text(Path::new(path));
        let params = SeqRenamingParameters::RnId(names);
        rename_seq!(self, params);
    }

    // Remove a string from the IDs.
    pub(crate) fn remove_string(&self, input_str: &str) {
        let params = SeqRenamingParameters::RmStr(input_str.to_string());
        rename_seq!(self, params);
    }

    // Remove the first regex match from the IDs, or all matches if `all` is true.
    #[pyo3(signature = (regex, all=false))]
    pub(crate) fn remove_regex(&self, regex: &str, all: bool) {
        let params = SeqRenamingParameters::RmRegex(regex.to_string(), all);
        rename_seq!(self, params);
    }

    // Replace a string in the IDs.
    pub(crate) fn replace_string(&self, from: &str, to: &str) {
        let params = SeqRenamingParameters::RpStr(from.to_string(), to.to_string());
        rename_seq!(self, params);
    }

    // Replace the first regex match in the IDs, or all matches if `all` is true.
    #[pyo3(signature = (regex, to, all=false))]
    pub(crate) fn replace_regex(&self, regex: &str, to: &str, all: bool) {
        let params = SeqRenamingParameters::RpRegex(regex.to_string(), to.to_string(), all);
        rename_seq!(self, params);
    }
}
