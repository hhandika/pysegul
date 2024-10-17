//! Rename sequence files on many alignments at once.

use pyo3::prelude::*;

#[pyclass]
pub(crate) struct SequenceRenaming {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_format: OutputFmt,
}
