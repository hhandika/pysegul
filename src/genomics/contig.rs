use std::path::{Path, PathBuf};

use pyo3::{prelude::*, types::PyDict};
use segul_core::{
    core::contig::summarize::ContigSummaryHandler,
    helper::{finder::ContigFileFinder, types::ContigFmt},
    plot::contig::plot_contig_summary,
};

use crate::common::{output_dict, parse, to_paths, to_py_err};
use crate::plot::{write_plots, PlotOptions};

const INPUT_FMT_ERR: &str = "Invalid input format. Valid options: 'auto', 'fasta', 'gzip'";

/// Summarize contig assemblies.
///
/// input_fmt: 'auto', 'fasta', or 'gzip'.
/// output_dir: Output directory.
/// output_prefix: Name of the output files.
/// plot: Chart options. Writes charts when set.
///
/// The run methods return a dict of output paths: 'summary', 'report',
/// and 'plots' (a list).
#[pyclass]
pub(crate) struct ContigSummary {
    input_files: Vec<PathBuf>,
    input_fmt: ContigFmt,
    output_dir: PathBuf,
    output_prefix: Option<String>,
    plot: Option<PlotOptions>,
}

#[pymethods]
impl ContigSummary {
    #[new]
    #[pyo3(signature = (input_fmt, output_dir, output_prefix=None, plot=None))]
    fn new(
        input_fmt: &str,
        output_dir: &str,
        output_prefix: Option<String>,
        plot: Option<PlotOptions>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, INPUT_FMT_ERR)?,
            output_dir: PathBuf::from(output_dir),
            output_prefix,
            plot,
        })
    }

    /// Summarize the given contig files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.summarize_contigs(py)
    }

    /// Summarize the contig files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = ContigFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
        self.summarize_contigs(py)
    }
}

impl ContigSummary {
    fn summarize_contigs<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let prefix = self.output_prefix.as_deref();
        let files =
            ContigSummaryHandler::new(&self.input_files, &self.input_fmt, &self.output_dir, prefix)
                .try_summarize()
                .map_err(to_py_err)?;
        let plots = write_plots(self.plot.as_ref(), &self.output_dir, prefix, |writer| {
            plot_contig_summary(&files.summary, writer)
        })?;
        let outputs = output_dict(
            py,
            &[
                ("summary", Some(&files.summary)),
                ("report", Some(&files.report)),
            ],
        )?;
        outputs.set_item("plots", plots)?;
        Ok(outputs)
    }
}
