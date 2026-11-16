//! Translate DNA to amino acids.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::sequence::translate::SequenceTranslation as Translate,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, GeneticCodes, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR, parse, to_paths};

const TRANSLATION_TABLE_ERR: &str =
    "Invalid translation table. Visit: https://www.segul.app/docs/cli-usage/translate";

/// Translate DNA sequences to amino acids.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_format: 'fasta', 'nexus', 'phylip', or their interleaved forms.
/// table: NCBI genetic code number, e.g. '1' for the standard code.
/// reading_frame: 1, 2, or 3. Detected for each file when not set.
#[pyclass]
pub(crate) struct SequenceTranslation {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: OutputFmt,
    translation_table: GeneticCodes,
    reading_frame: Option<usize>,
}

#[pymethods]
impl SequenceTranslation {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_format, table="1", reading_frame=None))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_format: &str,
        table: &str,
        reading_frame: Option<usize>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_format: parse(output_format, SEQ_OUTPUT_FMT_ERR)?,
            translation_table: parse(table, TRANSLATION_TABLE_ERR)?,
            reading_frame,
        })
    }

    /// Translate the given sequence files.
    fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
        self.translate();
    }

    /// Translate the sequence files in a directory.
    fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.translate();
    }
}

impl SequenceTranslation {
    fn translate(&self) {
        let handle = Translate::new(
            &self.input_fmt,
            &self.translation_table,
            &self.datatype,
            &self.output_format,
        );
        match self.reading_frame {
            Some(frame) => handle.translate(&self.input_files, frame, &self.output_path),
            None => handle.translate_all_autoframe(&self.input_files, &self.output_path),
        }
    }
}
