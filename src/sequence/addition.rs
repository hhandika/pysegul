//! Add sequences to existing sequence files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::addition::SequenceAddition as Addition,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, parse, parse_fasta_fmt, to_paths};

/// Add sequences from the input files to destination files.
/// Sequences are matched to destination files by file name (without extension).
/// The output is unaligned: gaps and missing data are removed.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta' or 'fasta-int'.
/// added_only: Write only destination files that received new sequences.
#[pyclass]
pub(crate) struct SequenceAddition {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
    added_only: bool,
}

#[pymethods]
impl SequenceAddition {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta", added_only=false))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        added_only: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse_fasta_fmt(output_fmt)?,
            added_only,
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

    /// Add sequences to the destination files.
    /// dest_fmt: Destination file format. Options: 'auto', 'fasta', 'nexus', 'phylip'.
    #[pyo3(signature = (dest_files, dest_fmt="auto"))]
    fn add_to_files(&self, dest_files: Vec<String>, dest_fmt: &str) -> PyResult<()> {
        let dest_fmt: InputFmt = parse(dest_fmt, SEQ_INPUT_FMT_ERR)?;
        self.handle().add(&to_paths(dest_files), &dest_fmt);
        Ok(())
    }

    /// Add sequences to the files in the destination directory.
    #[pyo3(signature = (dest_dir, dest_fmt="auto"))]
    fn add_to_dir(&self, dest_dir: &str, dest_fmt: &str) -> PyResult<()> {
        let dest_fmt: InputFmt = parse(dest_fmt, SEQ_INPUT_FMT_ERR)?;
        let dest_files = SeqFileFinder::new(Path::new(dest_dir)).find(&dest_fmt);
        self.handle().add(&dest_files, &dest_fmt);
        Ok(())
    }

    /// Add all input sequences into a single file.
    /// include_filename: Append the input file name to each sequence ID.
    #[pyo3(signature = (dest_file, include_filename=false))]
    fn add_to_file(&self, dest_file: &str, include_filename: bool) {
        self.handle()
            .add_single(Path::new(dest_file), include_filename);
    }
}

impl SequenceAddition {
    fn handle(&self) -> Addition<'_> {
        Addition::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &self.output_fmt,
            self.added_only,
        )
    }
}
