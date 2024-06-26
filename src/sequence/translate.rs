//! Translate DNA to amino acids.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::sequence::translate::Translate,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, GeneticCodes, InputFmt, OutputFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

const TRANSLATION_TABLE_ERR: &str =
    "Invalid translation table. Visit: https://www.segul.app/docs/cli-usage/translate";

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
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_format: &str,
        table: &str,
        reading_frame: Option<usize>,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype: datatype.parse().expect(SEQ_DATA_TYPE_ERR),
            output_path: PathBuf::from(output_path),
            output_format: output_format.parse().expect(SEQ_OUTPUT_FMT_ERR),
            translation_table: table.parse().expect(TRANSLATION_TABLE_ERR),
            reading_frame,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.translate();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.translate();
    }

    // Translate from files
    fn translate(&self) {
        let handle = Translate::new(
            &self.translation_table,
            &self.input_fmt,
            &self.datatype,
            &self.output_format,
        );
        match self.reading_frame {
            Some(frame) => handle.translate_all(&self.input_files, frame, &self.output_path),
            None => handle.translate_all_autoframe(&self.input_files, &self.output_path),
        }
    }
}
