//! Convert alignments to unaligned sequences by removing gaps and missing data.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::unalign::UnalignAlignment,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, parse, parse_fasta_fmt, to_paths};

/// Remove gaps and missing data from alignments.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta' or 'fasta-int'.
#[pyclass]
pub(crate) struct AlignmentUnalignment {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
}

#[pymethods]
impl AlignmentUnalignment {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta"))]
    fn new(input_fmt: &str, datatype: &str, output_path: &str, output_fmt: &str) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse_fasta_fmt(output_fmt)?,
        })
    }

    /// Unalign the given alignment files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.unalign();
    }

    /// Unalign the alignment files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.unalign();
    }
}

impl AlignmentUnalignment {
    fn unalign(&self) {
        UnalignAlignment::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &self.output_fmt,
        )
        .unalign();
    }
}
