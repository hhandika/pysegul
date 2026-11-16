//! Compute pairwise genetic distances for many alignments at once.

use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use segul_core::{
    core::align::distance::{
        AlignmentDistance as Distance, MatrixDiagonal, MatrixTriangle, group::SampleGrouping,
    },
    helper::{
        finder::{IDs, SeqFileFinder},
        types::{DataType, InputFmt},
    },
    plot::distance::plot_distance,
    stats::distance::DistanceModel,
};

use crate::common::{
    SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, output_dict, parse, to_paths, to_py_err,
};
use crate::plot::{PlotOptions, write_plots};

const MODEL_ERR: &str =
    "Invalid distance model. Valid options: 'p', 'jc69', 'k2p', 'tn93', 'poisson'";
const TRIANGLE_ERR: &str = "Invalid matrix triangle. Valid options: 'full', 'lower', 'upper'";
const DIAGONAL_ERR: &str =
    "Invalid matrix diagonal. Valid options: 'zero', 'blank', 'na', 'mean', 'max'";

// How samples are assigned to groups for group mean distances.
enum Grouping {
    None,
    Ready(SampleGrouping),
    // Detected from sample IDs when computing.
    Auto,
}

/// Compute pairwise genetic distances for each alignment.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna' or 'aa'.
/// output_path: Output directory.
/// model: 'p', 'jc69', 'k2p', 'tn93' for DNA. 'p', 'jc69', 'poisson' for amino acids.
/// triangle: Part of the matrix to write. 'full', 'lower', 'upper'.
/// diagonal: Matrix diagonal values. 'zero', 'blank', 'na'. With grouping, 'mean' or 'max'
///     writes the within-group mean or highest distance on the group matrices.
///     Defaults to zero, and the within-group mean on the group matrices.
/// long_format: Also write long-format tables with one row per pair.
/// output_prefix: Name of the output files.
/// plot: Chart options. Writes charts when set.
///
/// The run methods return a dict of output paths: 'summary', 'per_locus_dir',
/// 'group_mean', 'pairwise_long', 'report', and 'plots' (a list).
#[pyclass]
pub(crate) struct AlignmentDistance {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    model: DistanceModel,
    triangle: MatrixTriangle,
    diagonal: Option<MatrixDiagonal>,
    long_format: bool,
    output_prefix: Option<String>,
    grouping: Grouping,
    plot: Option<PlotOptions>,
}

#[pymethods]
impl AlignmentDistance {
    #[new]
    #[pyo3(signature = (
        input_fmt,
        datatype,
        output_path,
        model="p",
        triangle="full",
        diagonal=None,
        long_format=false,
        output_prefix=None,
        plot=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        model: &str,
        triangle: &str,
        diagonal: Option<&str>,
        long_format: bool,
        output_prefix: Option<String>,
        plot: Option<PlotOptions>,
    ) -> PyResult<Self> {
        let datatype: DataType = parse(datatype, SEQ_DATA_TYPE_ERR)?;
        let model: DistanceModel = parse(model, MODEL_ERR)?;
        if !model.is_valid_for(&datatype) {
            return Err(PyValueError::new_err(format!(
                "The {} model is not valid for {} data. \
                Use p, jc69, k2p, or tn93 for DNA, and p, jc69, or poisson for amino acid.",
                model, datatype
            )));
        }
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype,
            output_path: PathBuf::from(output_path),
            model,
            triangle: parse(triangle, TRIANGLE_ERR)?,
            diagonal: diagonal.map(|d| parse(d, DIAGONAL_ERR)).transpose()?,
            long_format,
            output_prefix,
            grouping: Grouping::None,
            plot,
        })
    }

    /// Group samples by the first `fields` fields of their IDs split on `delimiter`.
    fn group_by_delimiter(&mut self, delimiter: char, fields: usize) -> PyResult<()> {
        let grouping = SampleGrouping::from_delimiter(delimiter, fields).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    /// Group samples by the first capture group of a regular expression
    /// (or the whole match if there is no capture group).
    fn group_by_regex(&mut self, pattern: &str) -> PyResult<()> {
        let grouping = SampleGrouping::from_regex(pattern).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    /// Group samples using a two-column CSV or TSV file (sample ID, group)
    /// with a header row.
    fn group_by_file(&mut self, path: &str) -> PyResult<()> {
        let grouping = SampleGrouping::from_map_file(Path::new(path)).map_err(to_py_err)?;
        self.grouping = Grouping::Ready(grouping);
        Ok(())
    }

    /// Detect groups from the sample IDs when computing.
    fn group_auto(&mut self) {
        self.grouping = Grouping::Auto;
    }

    /// Compute distances for the given alignment files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.compute(py)
    }

    /// Compute distances for the alignment files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.compute(py)
    }
}

impl AlignmentDistance {
    fn compute<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let needs_grouping = self.diagonal.is_some_and(|d| d.requires_grouping());
        if needs_grouping && matches!(self.grouping, Grouping::None) {
            return Err(PyValueError::new_err(
                "The 'mean' and 'max' diagonals require grouping. \
                Call a group_by method or group_auto first.",
            ));
        }
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
        .with_long_format(self.long_format);
        if let Some(diagonal) = self.diagonal {
            handle = handle.with_diagonal(diagonal);
        }
        if let Grouping::Ready(grouping) = &self.grouping {
            handle = handle.with_grouping(grouping.clone());
        }
        let prefix = self.output_prefix.as_deref();
        let files = handle.compute(&self.input_files, prefix);
        let plots = write_plots(self.plot.as_ref(), &self.output_path, prefix, |writer| {
            plot_distance(&files, writer)
        })?;
        let outputs = output_dict(
            py,
            &[
                ("summary", Some(&files.summary)),
                ("per_locus_dir", Some(&files.per_locus_dir)),
                ("group_mean", files.group_mean.as_deref()),
                ("pairwise_long", files.pairwise_long.as_deref()),
                ("report", Some(&files.report)),
            ],
        )?;
        outputs.set_item("plots", plots)?;
        Ok(outputs)
    }

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
