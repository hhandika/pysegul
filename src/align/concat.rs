use std::path::{Path, PathBuf};

use pyo3::prelude::*;

use segul_core::{
    core::align::concat::AlignmentConcatenation as ConcatHandler,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR, parse, to_paths};

use super::PARTITION_FMT_ERR;

/// Concatenate multiple sequence alignments into a single alignment
/// and write a partition file for it.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_dir: Directory for the concatenated alignment.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms
///     ('fasta-int', 'nexus-int', 'phylip-int').
/// partition_fmt: 'charset', 'nexus', or 'raxml'. Add '-codon' for codon partitions.
/// output_prefix: Name of the output files.
#[pyclass]
pub(crate) struct AlignmentConcatenation {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_dir: PathBuf,
    output_fmt: OutputFmt,
    partition_fmt: PartitionFmt,
    output_prefix: Option<String>,
}

#[pymethods]
impl AlignmentConcatenation {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_dir, output_fmt, partition_fmt, output_prefix=None))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_dir: &str,
        output_fmt: &str,
        partition_fmt: &str,
        output_prefix: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_dir: PathBuf::from(output_dir),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
            partition_fmt: parse(partition_fmt, PARTITION_FMT_ERR)?,
            output_prefix,
        })
    }

    /// Concatenate the given alignment files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.concat_alignments();
    }

    /// Concatenate the alignment files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.concat_alignments();
    }
}

impl AlignmentConcatenation {
    fn concat_alignments(&mut self) {
        let prefix = match &self.output_prefix {
            Some(prefix) => PathBuf::from(prefix),
            None => self.output_dir.clone(),
        };
        let mut handle = ConcatHandler::new(
            &self.input_fmt,
            &self.output_dir,
            &self.output_fmt,
            &self.partition_fmt,
            &prefix,
        );
        handle.concat(&mut self.input_files, &self.datatype);
    }
}
