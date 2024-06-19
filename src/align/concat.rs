use std::path::Path;

use pyo3::prelude::*;
use segul::{
    handler::align::concat::ConcatHandler,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

#[pyfunction]
pub(crate) fn concat_alignments(
    input_fmt: &str,
    input_dir: &str,
    datatype: &str,
    output_path: &str,
    output_fmt: &str,
    partition_fmt: &str,
) {
    let fmt = input_fmt.parse::<InputFmt>().expect("Invalid input format");
    let dir = Path::new(input_dir);
    let output = Path::new(output_path);
    let datatype = datatype.parse::<DataType>().expect("Invalid data type");
    let out_fmt = output_fmt
        .parse::<OutputFmt>()
        .expect("Invalid output format");
    let part_fmt = partition_fmt
        .parse::<PartitionFmt>()
        .expect("Invalid partition format");
    let mut files = SeqFileFinder::new(dir).find(&fmt);
    let mut handle = ConcatHandler::new(&fmt, &output, &out_fmt, &part_fmt);
    handle.concat_alignment(&mut files, &datatype);
}
