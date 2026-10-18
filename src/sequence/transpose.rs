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

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

// Each output file contains all loci for a single sample,
// written to `output_path/<sample_id>/<sample_id>.<ext>`.
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
    // output_fmt: Use 'fasta' or 'fasta-int'. Transposed sequences are unaligned.
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta"))]
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

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.transpose();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.transpose();
    }

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
