use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::convert::AlignmentConversion as Converter,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

use crate::common::{parse, to_paths, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

/// Convert alignments to another format.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
/// sort_sequence: Sort sequences by ID.
#[pyclass]
pub(crate) struct AlignmentConversion {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
    sort_sequence: bool,
}

#[pymethods]
impl AlignmentConversion {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt, sort_sequence=false))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        sort_sequence: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
            sort_sequence,
        })
    }

    /// Convert the given alignment files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.convert_alignments();
    }

    /// Convert the alignment files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.convert_alignments();
    }
}

impl AlignmentConversion {
    fn convert_alignments(&self) {
        let handle = Converter::new(
            &self.input_fmt,
            &self.output_fmt,
            &self.datatype,
            self.sort_sequence,
        );
        handle.convert(&self.input_files, &self.output_path);
    }
}
