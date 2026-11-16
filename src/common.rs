//! Shared error messages and conversions between Python and segul-core.

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use segul_core::helper::types::OutputFmt;

pub const SEQ_OUTPUT_FMT_ERR: &str = "Invalid output format. \
    Valid options: 'fasta', 'nexus', 'phylip', \
    'fasta-int', 'nexus-int', 'phylip-int'";

pub const SEQ_INPUT_FMT_ERR: &str =
    "Invalid input format. Valid options: 'auto', 'fasta', 'nexus', 'phylip'";
pub const SEQ_DATA_TYPE_ERR: &str = "Invalid data type. Valid options: 'dna', 'aa', 'ignore'";
pub const FASTA_OUTPUT_FMT_ERR: &str = "Invalid output format. Valid options: 'fasta', 'fasta-int'";

/// Parse a user option, raising `ValueError` with `err` when it is invalid.
pub fn parse<T: FromStr>(value: &str, err: &str) -> PyResult<T> {
    value
        .parse()
        .map_err(|_| PyValueError::new_err(format!("{} Got '{}'.", err, value)))
}

/// Parse a FASTA output format, the only format for unaligned output.
pub fn parse_fasta_fmt(value: &str) -> PyResult<OutputFmt> {
    match value {
        "fasta" => Ok(OutputFmt::Fasta),
        "fasta-int" => Ok(OutputFmt::FastaInt),
        _ => Err(PyValueError::new_err(format!(
            "{} Got '{}'.",
            FASTA_OUTPUT_FMT_ERR, value
        ))),
    }
}

pub fn to_py_err<E: Display>(e: E) -> PyErr {
    PyValueError::new_err(format!("{:#}", e))
}

pub fn to_paths(files: Vec<String>) -> Vec<PathBuf> {
    files.into_iter().map(PathBuf::from).collect()
}

pub fn path_str(path: &Path) -> String {
    path.display().to_string()
}

/// Output paths returned to Python as a dict of name to path.
/// Missing outputs are `None`.
pub fn output_dict<'py>(
    py: Python<'py>,
    outputs: &[(&str, Option<&Path>)],
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (name, path) in outputs {
        dict.set_item(name, path.map(path_str))?;
    }
    Ok(dict)
}
