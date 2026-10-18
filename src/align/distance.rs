//! Compute pairwise genetic distances for many alignments at once.

use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*};
use segul_core::{
    core::align::distance::{
        group::SampleGrouping, AlignmentDistance as Distance, GroupDiagonal, MatrixDiagonal,
        MatrixTriangle,
    },
    helper::{
        finder::{IDs, SeqFileFinder},
        types::{DataType, InputFmt},
    },
    stats::distance::DistanceModel,
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR};

const MODEL_ERR: &str =
    "Invalid distance model. Valid options: 'p', 'jc69', 'k2p', 'tn93', 'poisson'";
const TRIANGLE_ERR: &str = "Invalid matrix triangle. Valid options: 'full', 'lower', 'upper'";
const DIAGONAL_ERR: &str = "Invalid matrix diagonal. Valid options: 'zero', 'blank', 'max'";
const GROUP_DIAGONAL_ERR: &str =
    "Invalid group matrix diagonal. Valid options: 'within', 'zero', 'blank', 'max'";

// How samples are assigned to groups for group mean distances.
enum Grouping {
    None,
    Ready(SampleGrouping),
    // Detected from sample IDs when computing.
    Auto,
}

#[pyclass]
pub(crate) struct AlignmentDistance {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    model: DistanceModel,
    triangle: MatrixTriangle,
    diagonal: MatrixDiagonal,
    group_diagonal: GroupDiagonal,
    long_format: bool,
    output_prefix: Option<String>,
    grouping: Grouping,
}

#[pymethods]
impl AlignmentDistance {
    // model: 'p', 'jc69', 'k2p', 'tn93' for DNA. 'p', 'jc69', 'poisson' for amino acids.
    // triangle: Part of the matrix to write. 'full', 'lower', 'upper'.
    // diagonal: Matrix diagonal values. 'zero', 'blank', 'max' (row maximum).
    // group_diagonal: Group matrix diagonal values. 'within', 'zero', 'blank', 'max'.
    // long_format: Also write long-format tables with one row per pair.
    #[new]
    #[pyo3(signature = (
        input_fmt,
        datatype,
        output_path,
        model="p",
        triangle="full",
        diagonal="zero",
        group_diagonal="within",
        long_format=false,
        output_prefix=None
    ))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        model: &str,
        triangle: &str,
        diagonal: &str,
        group_diagonal: &str,
        long_format: bool,
        output_prefix: Option<String>,
    ) -> PyResult<Self> {
        let datatype: DataType = datatype.parse().expect(SEQ_DATA_TYPE_ERR);
        let model: DistanceModel = model.parse().expect(MODEL_ERR);
        if !model.is_valid_for(&datatype) {
            return Err(PyValueError::new_err(format!(
                "The {} model is not valid for {} data. \
                Use p, jc69, k2p, or tn93 for DNA, and p, jc69, or poisson for amino acid.",
                model, datatype
            )));
        }
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: input_fmt.parse().expect(SEQ_INPUT_FMT_ERR),
            datatype,
            output_path: PathBuf::from(output_path),
            model,
            triangle: triangle.parse().expect(TRIANGLE_ERR),
            diagonal: diagonal.parse().expect(DIAGONAL_ERR),
            group_diagonal: group_diagonal.parse().expect(GROUP_DIAGONAL_ERR),
            long_format,
            output_prefix,
            grouping: Grouping::None,
        })
    }

    // Group samples by the first `fields` fields of their IDs split on `delimiter`.
    pub(crate) fn group_by_delimiter(&mut self, delimiter: char, fields: usize) -> PyResult<()> {
        let grouping = SampleGrouping::from_delimiter(delimiter, fields).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    // Group samples by the first capture group of a regular expression
    // (or the whole match if there is no capture group).
    pub(crate) fn group_by_regex(&mut self, pattern: &str) -> PyResult<()> {
        let grouping = SampleGrouping::from_regex(pattern).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    // Group samples using a two-column CSV or TSV file (sample ID, group)
    // with a header row.
    pub(crate) fn group_by_file(&mut self, path: &str) -> PyResult<()> {
        let grouping = SampleGrouping::from_map_file(Path::new(path)).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    // Detect groups from the sample IDs when computing.
    pub(crate) fn group_auto(&mut self) {
        self.grouping = Grouping::Auto;
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) -> PyResult<()> {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.compute()
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) -> PyResult<()> {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
        self.compute()
    }

    fn compute(&mut self) -> PyResult<()> {
        if let Grouping::Auto = self.grouping {
            self.grouping = Grouping::Ready(self.detect_grouping()?);
        }
        let mut handle = Distance::new(
            &self.input_fmt,
            &self.datatype,
            &self.model,
            &self.output_path,
        )
        .with_triangle(self.triangle)
        .with_diagonal(self.diagonal)
        .with_long_format(self.long_format);
        if let Grouping::Ready(grouping) = &self.grouping {
            handle = handle
                .with_grouping(grouping.clone())
                .with_group_diagonal(self.group_diagonal);
        }
        handle.compute(&self.input_files, self.output_prefix.as_deref());
        Ok(())
    }
}

impl AlignmentDistance {
    fn detect_grouping(&self) -> PyResult<SampleGrouping> {
        let ids: Vec<String> = IDs::new(&self.input_files, &self.input_fmt, &self.datatype)
            .id_unique()
            .into_iter()
            .collect();
        SampleGrouping::detect(&ids)
            .map(|suggestion| suggestion.to_grouping())
            .ok_or_else(|| {
                PyValueError::new_err(
                    "Could not detect groups from sample IDs. \
                    Use group_by_delimiter(), group_by_regex(), or group_by_file().",
                )
            })
    }
}

fn to_py_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyValueError::new_err(format!("{:#}", e))
}
