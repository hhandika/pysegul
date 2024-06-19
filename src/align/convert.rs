use std::path::Path;

use pyo3::prelude::*;
use segul::{
    handler::align::convert::Converter,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt},
    },
};

#[pyfunction]
pub(crate) fn convert_alignments(
    input_dir: &str,
    input_fmt: &str,
    datatype: &str,
    output_path: &str,
    output_fmt: &str,
    sort_sequence: bool,
) {
    let input_dir = Path::new(input_dir);
    let input_fmt = input_fmt.parse::<InputFmt>().expect("Invalid input format");
    let datatype = datatype.parse::<DataType>().expect("Invalid data type");
    let output_prefix = Path::new(output_path);
    let input_files = SeqFileFinder::new(input_dir).find(&input_fmt);
    let output_fmt = output_fmt
        .parse::<OutputFmt>()
        .expect("Invalid output format");
    let mut handle = Converter::new(&input_fmt, &output_fmt, &datatype, sort_sequence);
    handle.convert(&input_files, output_prefix);
}
