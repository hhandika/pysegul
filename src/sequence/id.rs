//! Extract unique IDs across many sequenc files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::id::SequenceID,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
};

use crate::common::{parse, to_paths, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

/// Extract unique IDs across many sequence files.
/// Can also write a file mapping each ID to the files that contain it.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// map_id: Also write the ID map.
/// output_prefix: Name of the output files.
#[pyclass]
pub(crate) struct IDExtraction {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_prefix: Option<String>,
    map_id: bool,
}

#[pymethods]
impl IDExtraction {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, map_id=false, output_prefix=None))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        map_id: bool,
        output_prefix: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_prefix,
            map_id,
        })
    }

    /// Extract IDs from the given sequence files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.extract();
    }

    /// Extract IDs from the sequence files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.extract();
    }
}

impl IDExtraction {
    fn extract(&self) {
        let handle = SequenceID::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            self.output_prefix.as_deref(),
        );
        // Mapping also writes the unique IDs.
        if self.map_id {
            handle.map_id();
        } else {
            handle.get_unique();
        }
    }
}
