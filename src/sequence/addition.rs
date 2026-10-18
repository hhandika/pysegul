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

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

const ADD_OUTPUT_FMT_ERR: &str = "Invalid output format. Valid options: 'fasta', 'fasta-int'";

// Add sequences from the input files to destination files.
// Sequences are matched to destination files by file name (without extension).
// The output is unaligned: gaps and missing data are removed.
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
    // output_fmt: Only 'fasta' and 'fasta-int' are supported.
    // added_only: Write only destination files that received new sequences.
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt="fasta", added_only=false))]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        added_only: bool,
    ) -> Self {
        let output_fmt = match output_fmt {
            "fasta" => OutputFmt::Fasta,
            "fasta-int" => OutputFmt::FastaInt,
            _ => panic!("{}", ADD_OUTPUT_FMT_ERR),
        };
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_fmt,
            added_only,
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

    // Add sequences to the destination files.
    // dest_fmt: Destination file format. Options: 'auto', 'fasta', 'nexus', 'phylip'.
    pub(crate) fn add_to_files(&self, dest_files: Vec<String>, dest_fmt: &str) {
        let dest_files: Vec<PathBuf> = dest_files.iter().map(PathBuf::from).collect();
        let dest_fmt: InputFmt = dest_fmt.parse().expect(SEQ_INPUT_FMT_ERR);
        self.handle().add(&dest_files, &dest_fmt);
    }

    // Add sequences to the files in the destination directory.
    pub(crate) fn add_to_dir(&self, dest_dir: &str, dest_fmt: &str) {
        let dest_fmt: InputFmt = dest_fmt.parse().expect(SEQ_INPUT_FMT_ERR);
        let dest_files = SeqFileFinder::new(Path::new(dest_dir)).find(&dest_fmt);
        self.handle().add(&dest_files, &dest_fmt);
    }

    // Add all input sequences into a single file.
    // include_filename: Append the input file name to each sequence ID.
    #[pyo3(signature = (dest_file, include_filename=false))]
    pub(crate) fn add_to_file(&self, dest_file: &str, include_filename: bool) {
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
