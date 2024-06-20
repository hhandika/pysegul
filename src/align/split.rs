use std::path::Path;

use pyo3::prelude::*;
use segul::helper::{
    finder::SeqFileFinder,
    types::{DataType, InputFmt, OutputFmt, PartitionFmt},
};

#[pyfunction]
#[allow(dead_code, unused_variables)]
pub(crate) fn split_alignments(
    input_dir: &str,
    input_fmt: &str,
    datatype: &str,
    output_dir: &str,
    output_fmt: &str,
    partition_fmt: &str,
    uncheck_partition: bool,
    input_partition: Option<String>,
    output_prefix: Option<String>,
) {
    let input_fmt = input_fmt
        .parse::<InputFmt>()
        .expect("Invalid input format. Valid options: 'fasta', 'nexus', 'phylip'");
    let input_dir = Path::new(input_dir);
    let output_dir = Path::new(output_dir);
    let datatype = datatype.parse::<DataType>().expect("Invalid data type");
    let out_fmt = output_fmt
        .parse::<OutputFmt>()
        .expect("Invalid output format");
    let part_fmt = partition_fmt
        .parse::<PartitionFmt>()
        .expect("Invalid partition format");
    let files = SeqFileFinder::new(input_dir).find(&input_fmt);
}
