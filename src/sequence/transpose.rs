//! Transpose locus alignments into per-sample sequence files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::transpose::SequenceTranspose as Transpose,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{parse, to_paths, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

/// Transpose locus alignments into per-sample sequence files.
/// Each output file contains all loci for a single sample,
/// written to `output_path/<sample_id>/<sample_id>.<ext>`.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta' or 'fasta-int'. Transposed sequences are unaligned.
#[pyclass]
pub(crate) struct SequenceTranspose {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
}

#[pymethods]
impl SequenceTranspose {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta"))]
    fn new(input_fmt: &str, datatype: &str, output_path: &str, output_fmt: &str) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
        })
    }

    /// Transpose the given alignment files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.transpose();
    }

    /// Transpose the alignment files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.transpose();
    }
}

impl SequenceTranspose {
    fn transpose(&self) {
        Transpose::new(
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &self.output_fmt,
        )
        .transpose(&self.input_files);
    }
}
