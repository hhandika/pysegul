//! Extract unique IDs across many sequenc files.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::sequence::id::Id,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

// Extract unique IDs across many sequence files.
// Can also generate a mapping file for the extracted IDs.
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
    // Create a new IDExtraction instance.
    // input_fmt: Input sequence format. Options: 'fasta', 'phylip', 'nexus'.
    // datatype: Data type of the sequence. Options: 'dna', 'aa'.
    // output_path: Directory to save the extracted IDs.
    // map_id: Generate a mapping file for the extracted IDs if true.
    // output_prefix: Prefix for the output files.
    #[new]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        map_id: bool,
        output_prefix: Option<String>,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<InputFmt>().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse::<DataType>().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_prefix,
            map_id,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.extract();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.extract();
    }

    fn extract(&mut self) {
        let handle = Id::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            self.output_prefix.as_deref(),
        );
        handle.generate_id();

        if self.map_id {
            handle.map_id();
        }
    }
}
