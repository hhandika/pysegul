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

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

const UNALIGN_OUTPUT_FMT_ERR: &str = "Invalid output format. Valid options: 'fasta', 'fasta-int'";

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
    // output_fmt: Only 'fasta' and 'fasta-int' are supported.
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta"))]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
    ) -> Self {
        let output_fmt = match output_fmt {
            "fasta" => OutputFmt::Fasta,
            "fasta-int" => OutputFmt::FastaInt,
            _ => panic!("{}", UNALIGN_OUTPUT_FMT_ERR),
        };
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_fmt,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.unalign();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.unalign();
    }

    fn unalign(&self) {
        let handle = UnalignAlignment::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &self.output_fmt,
        );
        handle.unalign();
    }
}
