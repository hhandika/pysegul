use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::align::convert::Converter,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

#[pyclass]
pub struct AlignmentConversion {
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
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        sort_sequence: bool,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse::<InputFmt>().expect("Invalid input format"),
            datatype: datatype.parse::<DataType>().expect("Invalid data type"),
            output_path: PathBuf::from(output_path),
            output_fmt: output_fmt
                .parse::<OutputFmt>()
                .expect("Invalid output format"),
            sort_sequence,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.convert_alignments();
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.convert_alignments();
    }

    fn convert_alignments(&self) {
        let mut handle = Converter::new(
            &self.input_fmt,
            &self.output_fmt,
            &self.datatype,
            self.sort_sequence,
        );
        handle.convert(&self.input_files, &self.output_path);
    }
}
