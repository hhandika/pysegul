use std::path::Path;

use pyo3::prelude::*;
use segul::{
    handler::align::summarize::SeqStats,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
};

#[pyfunction]
pub(crate) fn summarize_alignments(
    input_dir: &str,
    input_fmt: &str,
    datatype: &str,
    output_path: &str,
    output_prefix: &str,
    summary_interval: usize,
) {
    let input_dir = Path::new(input_dir);
    let input_fmt = input_fmt.parse::<InputFmt>().expect("Invalid input format");
    let output_path = Path::new(output_path);
    let datatype = datatype.parse::<DataType>().expect("Invalid data type");
    let mut files = SeqFileFinder::new(input_dir).find(&input_fmt);
    let mut handle = SeqStats::new(&input_fmt, &output_path, summary_interval, &datatype);
    handle.summarize_all(&mut files, Some(output_prefix).as_deref());
}
