use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use segul_core::{
    core::align::summarize::AlignmentSummary as SeqStats,
    helper::{
        finder::SeqFileFinder,
        types::{DataType, InputFmt},
    },
    plot::align::plot_alignment_summary,
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, output_dict, parse, to_paths};
use crate::plot::{PlotOptions, write_plots};

const INTERVAL_ERR: &str = "Invalid summary interval. Valid options: 1, 2, 5, 10";

/// Summarize alignments.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// summary_interval: Step of the missing data summary, in percent. 1, 2, 5, or 10.
/// output_prefix: Name of the output files.
/// per_locus: Write a summary for each alignment
///     instead of a summary across all alignments.
/// plot: Chart options. Writes charts when set. Not available per locus.
///
/// The run methods return a dict of output paths: 'locus', 'taxon',
/// 'text', 'report', and 'plots' (a list). Per locus, the dict is empty.
#[pyclass]
pub(crate) struct AlignmentSummary {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    summary_interval: usize,
    output_prefix: Option<String>,
    per_locus: bool,
    plot: Option<PlotOptions>,
}

#[pymethods]
impl AlignmentSummary {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, summary_interval=5, output_prefix=None, per_locus=false, plot=None))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        summary_interval: usize,
        output_prefix: Option<String>,
        per_locus: bool,
        plot: Option<PlotOptions>,
    ) -> PyResult<Self> {
        if ![1, 2, 5, 10].contains(&summary_interval) {
            return Err(PyValueError::new_err(format!(
                "{} Got '{}'.",
                INTERVAL_ERR, summary_interval
            )));
        }
        if per_locus && plot.is_some() {
            return Err(PyValueError::new_err(
                "Charts are not available for per-locus summaries.",
            ));
        }
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            summary_interval,
            output_prefix,
            per_locus,
            plot,
        })
    }

    /// Summarize the given alignment files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.summarize_alignments(py)
    }

    /// Summarize the alignment files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.summarize_alignments(py)
    }
}

impl AlignmentSummary {
    fn summarize_alignments<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut handle = SeqStats::new(
            &self.input_fmt,
            &self.output_path,
            self.summary_interval,
            &self.datatype,
        );
        let prefix = self.output_prefix.as_deref();
        if self.per_locus {
            handle.summarize_locus(&self.input_files, prefix);
            return Ok(PyDict::new(py));
        }
        let files = handle.summarize_all(&self.input_files, prefix);
        let plots = write_plots(self.plot.as_ref(), &self.output_path, prefix, |writer| {
            plot_alignment_summary(&files, writer)
        })?;
        let outputs = output_dict(
            py,
            &[
                ("locus", Some(&files.locus)),
                ("taxon", Some(&files.taxon)),
                ("text", Some(&files.text)),
                ("report", Some(&files.report)),
            ],
        )?;
        outputs.set_item("plots", plots)?;
        Ok(outputs)
    }
}
