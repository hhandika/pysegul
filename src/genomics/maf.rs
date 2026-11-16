//! Convert and summarize multiple alignment format (MAF) files.

use std::path::{Path, PathBuf};

use pyo3::{prelude::*, types::PyDict};
use segul_core::{
    core::maf::{
        BlockFilter, SampleNaming,
        convert::{MafConverter, MafGrouping},
        summary::MafSummary as Summary,
    },
    helper::{finder::MafFileFinder, types::OutputFmt},
    plot::maf::plot_maf_summary,
};

use crate::common::{SEQ_OUTPUT_FMT_ERR, output_dict, parse, to_paths, to_py_err};
use crate::plot::{PlotOptions, write_plots};

const GROUP_BY_ERR: &str = "Invalid grouping. Valid options: 'block', 'chrom'";

fn parse_naming(full_source_name: bool) -> SampleNaming {
    if full_source_name {
        SampleNaming::FullSource
    } else {
        SampleNaming::AssemblyPrefix
    }
}

fn find_maf_files(input_dir: &str) -> Vec<PathBuf> {
    MafFileFinder::new(Path::new(input_dir)).find()
}

/// Convert MAF files to sequence alignments.
///
/// output_path: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
/// group_by: Without a BED file, write one file per 'block' or per reference 'chrom'.
/// bed: BED file of loci (.bed or .bed.gz). Blocks are assigned to every locus they overlap.
/// clip: Clip blocks to the loci. Set to False to keep whole blocks. Requires `bed`.
/// strand_aware: Reverse complement loci on the minus strand of the BED file. Requires `bed`.
/// full_source_name: Use full MAF source names (e.g. 'hg18.chr7') as sample names
///     instead of the assembly prefix ('hg18').
/// ref_species: Reference species. Defaults to the first species in the first block.
/// species: Keep only these species. The reference is still used for coordinates.
/// min_block_len: Skip blocks with fewer reference bases (gaps excluded).
/// min_species: Skip blocks with fewer species, counting the reference.
/// min_score: Skip blocks with a lower score. Blocks without a score pass.
/// write_unmatched: Write blocks that match no locus to the unmatched directory. Requires `bed`.
/// output_prefix: Name of the locus report CSV written with `bed`.
///
/// The run methods return a dict of block and locus counts.
#[pyclass]
pub(crate) struct MafConversion {
    input_files: Vec<PathBuf>,
    output_path: PathBuf,
    output_fmt: OutputFmt,
    group_by: MafGrouping,
    bed: Option<PathBuf>,
    clip: bool,
    strand_aware: bool,
    naming: SampleNaming,
    ref_species: Option<String>,
    filter: BlockFilter,
    write_unmatched: bool,
    output_prefix: Option<String>,
}

#[pymethods]
impl MafConversion {
    #[new]
    #[pyo3(signature = (
        output_path,
        output_fmt="fasta",
        group_by="block",
        bed=None,
        clip=true,
        strand_aware=false,
        full_source_name=false,
        ref_species=None,
        species=None,
        min_block_len=None,
        min_species=None,
        min_score=None,
        write_unmatched=false,
        output_prefix=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        output_path: &str,
        output_fmt: &str,
        group_by: &str,
        bed: Option<String>,
        clip: bool,
        strand_aware: bool,
        full_source_name: bool,
        ref_species: Option<String>,
        species: Option<Vec<String>>,
        min_block_len: Option<usize>,
        min_species: Option<usize>,
        min_score: Option<f64>,
        write_unmatched: bool,
        output_prefix: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            input_files: Vec::new(),
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
            group_by: parse(group_by, GROUP_BY_ERR)?,
            bed: bed.map(PathBuf::from),
            clip,
            strand_aware,
            naming: parse_naming(full_source_name),
            ref_species,
            filter: BlockFilter {
                min_ref_len: min_block_len,
                min_species,
                min_score,
                species,
            },
            write_unmatched,
            output_prefix,
        })
    }

    /// Convert the given MAF files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.convert(py)
    }

    /// Convert the MAF files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = find_maf_files(input_dir);
        self.convert(py)
    }
}

impl MafConversion {
    fn convert<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut converter =
            MafConverter::new(&self.input_files, &self.output_path, &self.output_fmt)
                .with_grouping(self.group_by)
                .with_clip(self.clip)
                .with_strand_aware(self.strand_aware)
                .with_naming(self.naming)
                .with_ref_species(self.ref_species.clone())
                .with_filter(self.filter.clone())
                .with_unmatched(self.write_unmatched)
                .with_prefix(self.output_prefix.clone());
        if let Some(bed) = &self.bed {
            converter = converter.with_bed(bed);
        }
        let report = converter.convert().map_err(to_py_err)?;
        let counts = PyDict::new(py);
        counts.set_item("files", report.files)?;
        counts.set_item("blocks_read", report.blocks_read)?;
        counts.set_item("blocks_no_ref", report.blocks_no_ref)?;
        counts.set_item("blocks_filtered", report.blocks_filtered)?;
        counts.set_item("blocks_unmatched", report.blocks_unmatched)?;
        counts.set_item("duplicate_rows", report.duplicate_rows)?;
        counts.set_item("overlapping_pieces", report.overlapping_pieces)?;
        counts.set_item("loci_written", report.loci_written)?;
        counts.set_item("loci_empty", report.loci_empty)?;
        Ok(counts)
    }
}

/// Summarize MAF files per file and per species.
///
/// output_path: Output directory.
/// output_prefix: Name of the output files.
/// full_source_name: Use full MAF source names as sample names.
/// ref_species: Reference species. Defaults to the first species in the first block.
/// species: Report only these species.
/// plot: Chart options. Writes charts when set.
///
/// The run methods return a dict of output paths: 'per_file',
/// 'per_species', and 'plots' (a list).
#[pyclass]
pub(crate) struct MafSummary {
    input_files: Vec<PathBuf>,
    output_path: PathBuf,
    output_prefix: Option<String>,
    naming: SampleNaming,
    ref_species: Option<String>,
    species: Option<Vec<String>>,
    plot: Option<PlotOptions>,
}

#[pymethods]
impl MafSummary {
    #[new]
    #[pyo3(signature = (
        output_path,
        output_prefix=None,
        full_source_name=false,
        ref_species=None,
        species=None,
        plot=None
    ))]
    fn new(
        output_path: &str,
        output_prefix: Option<String>,
        full_source_name: bool,
        ref_species: Option<String>,
        species: Option<Vec<String>>,
        plot: Option<PlotOptions>,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            output_path: PathBuf::from(output_path),
            output_prefix,
            naming: parse_naming(full_source_name),
            ref_species,
            species,
            plot,
        }
    }

    /// Summarize the given MAF files.
    fn from_files<'py>(
        &mut self,
        py: Python<'py>,
        input_files: Vec<String>,
    ) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = to_paths(input_files);
        self.summarize(py)
    }

    /// Summarize the MAF files in a directory.
    fn from_dir<'py>(&mut self, py: Python<'py>, input_dir: &str) -> PyResult<Bound<'py, PyDict>> {
        self.input_files = find_maf_files(input_dir);
        self.summarize(py)
    }
}

impl MafSummary {
    fn summarize<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let prefix = self.output_prefix.as_deref();
        let (per_file, per_species) = Summary::new(&self.input_files, &self.output_path, prefix)
            .with_naming(self.naming)
            .with_ref_species(self.ref_species.clone())
            .with_species(self.species.clone())
            .summarize()
            .map_err(to_py_err)?;
        let plots = write_plots(self.plot.as_ref(), &self.output_path, prefix, |writer| {
            plot_maf_summary(&per_file, &per_species, writer)
        })?;
        let outputs = output_dict(
            py,
            &[
                ("per_file", Some(&per_file)),
                ("per_species", Some(&per_species)),
            ],
        )?;
        outputs.set_item("plots", plots)?;
        Ok(outputs)
    }
}
