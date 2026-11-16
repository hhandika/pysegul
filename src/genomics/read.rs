use std::path::{Path, PathBuf};

use pyo3::{prelude::*, types::PyDict};
use segul_core::{
    core::read::summarize::GenomicReadSummary,
    helper::{
        finder::SeqReadFinder,
        types::{PhredOffset, SeqReadFmt, SummaryMode},
    },
    plot::read::plot_read_summary,
    stats::fastq::DEFAULT_DUP_SKETCH_SIZE,
};

use crate::common::{output_dict, parse, to_paths, to_py_err};
use crate::plot::{write_plots, PlotOptions};

const INPUT_FMT_ERR: &str = "Invalid input format. Valid options: 'auto', 'fastq', 'gzip'";
const SUMMARY_MODE_ERR: &str =
    "Invalid summary mode. Valid options: 'default', 'minimal', 'complete'";
const PHRED_ERR: &str = "Invalid phred offset. Valid options: 'auto', '33', '64'";

/// Summarize FASTQ reads.
///
/// input_fmt: 'auto', 'fastq', or 'gzip'.
/// mode: 'minimal', 'default', or 'complete'. Complete also writes
///     per-position statistics and a JSON report.
/// output_path: Output directory.
/// output_prefix: Name of the output files.
/// phred: Quality score offset. 'auto', '33', or '64'.
/// dup_sketch_size: Reads sampled to estimate duplication in complete mode.
/// plot: Chart options. Writes charts when set.
///
/// The run methods return a dict of output paths: 'summary', 'positions',
/// 'distributions', 'report', and 'plots' (a list).
#[pyclass]
pub(crate) struct ReadSummary {
    input_files: Vec<PathBuf>,
    input_fmt: SeqReadFmt,
    mode: SummaryMode,
    output_path: PathBuf,
    output_prefix: Option<String>,
    phred: PhredOffset,
    dup_sketch_size: usize,
    plot: Option<PlotOptions>,
}

#[pymethods]
impl ReadSummary {
    #[new]
    #[pyo3(signature = (
        input_fmt,
        mode,
        output_path,
        output_prefix=None,
        phred="auto",
        dup_sketch_size=DEFAULT_DUP_SKETCH_SIZE,
        plot=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        input_fmt: &str,
        mode: &str,
        output_path: &str,
        output_prefix: Option<String>,
        phred: &str,
        dup_sketch_size: usize,
        plot: Option<PlotOptions>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, INPUT_FMT_ERR)?,
            mode: parse(mode, SUMMARY_MODE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_prefix,
            phred: parse(phred, PHRED_ERR)?,
            dup_sketch_size,
            plot,
        })
    }

    /// Summarize the given read files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.summarize_reads(py)
    }

    /// Summarize the read files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = SeqReadFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.summarize_reads(py)
    }
}

impl ReadSummary {
    fn summarize_reads<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let prefix = self.output_prefix.as_deref();
        let files = GenomicReadSummary::new(
            &mut self.input_files,
            &self.input_fmt,
            &self.mode,
            &self.output_path,
            prefix,
        )
        .with_phred(self.phred)
        .with_dup_sketch_size(self.dup_sketch_size)
        .try_summarize()
        .map_err(to_py_err)?;
        let plots = write_plots(self.plot.as_ref(), &self.output_path, prefix, |writer| {
            plot_read_summary(&files, writer)
        })?;
        let outputs = output_dict(
            py,
            &[
                ("summary", Some(&files.summary)),
                ("positions", files.positions.as_deref()),
                ("distributions", files.distributions.as_deref()),
                ("report", files.report.as_deref()),
            ],
        )?;
        outputs.set_item("plots", plots)?;
        Ok(outputs)
    }
}
