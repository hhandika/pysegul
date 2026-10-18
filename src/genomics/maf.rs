//! Convert and summarize multiple alignment format (MAF) files.

use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*};
use segul_core::{
    core::maf::{
        convert::{MafConverter, MafGrouping},
        summary::MafSummary as Summary,
        BlockFilter, SampleNaming,
    },
    helper::{finder::MafFileFinder, types::OutputFmt},
};

use crate::common::SEQ_OUTPUT_FMT_ERR;

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

fn to_py_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyValueError::new_err(format!("{:#}", e))
}

// Convert MAF files to sequence alignments.
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
    // group_by: Without a BED file, write one file per 'block' or per reference 'chrom'.
    // bed: BED file of loci (.bed or .bed.gz). Blocks are assigned to every locus they overlap.
    // clip: Clip blocks to the loci. Set to False to keep whole blocks. Requires `bed`.
    // strand_aware: Reverse complement loci on the minus strand of the BED file. Requires `bed`.
    // full_source_name: Use full MAF source names (e.g. 'hg18.chr7') as sample names
    //     instead of the assembly prefix ('hg18').
    // ref_species: Reference species. Defaults to the first species in the first block.
    // species: Keep only these species. The reference is still used for coordinates.
    // min_block_len: Skip blocks with fewer reference bases (gaps excluded).
    // min_species: Skip blocks with fewer species, counting the reference.
    // min_score: Skip blocks with a lower score. Blocks without a score pass.
    // write_unmatched: Write blocks that match no locus to the unmatched directory. Requires `bed`.
    // output_prefix: Name of the locus report CSV written with `bed`.
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
    pub(crate) fn new(
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
    ) -> Self {
        Self {
            input_files: Vec::new(),
            output_path: PathBuf::from(output_path),
            output_fmt: output_fmt.parse().expect(SEQ_OUTPUT_FMT_ERR),
            group_by: group_by.parse().expect(GROUP_BY_ERR),
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
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) -> PyResult<()> {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.convert()
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) -> PyResult<()> {
        self.input_files = find_maf_files(input_dir);
        self.convert()
    }

    fn convert(&self) -> PyResult<()> {
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
        converter.convert().map_err(to_py_err)?;
        Ok(())
    }
}

// Summarize MAF files per species and per block.
#[pyclass]
pub(crate) struct MafSummary {
    input_files: Vec<PathBuf>,
    output_path: PathBuf,
    output_prefix: Option<String>,
    naming: SampleNaming,
    ref_species: Option<String>,
    species: Option<Vec<String>>,
}

#[pymethods]
impl MafSummary {
    #[new]
    #[pyo3(signature = (
        output_path,
        output_prefix=None,
        full_source_name=false,
        ref_species=None,
        species=None
    ))]
    pub(crate) fn new(
        output_path: &str,
        output_prefix: Option<String>,
        full_source_name: bool,
        ref_species: Option<String>,
        species: Option<Vec<String>>,
    ) -> Self {
        Self {
            input_files: Vec::new(),
            output_path: PathBuf::from(output_path),
            output_prefix,
            naming: parse_naming(full_source_name),
            ref_species,
            species,
        }
    }

    pub(crate) fn from_files(&mut self, input_files: Vec<String>) -> PyResult<()> {
        self.input_files = input_files.iter().map(PathBuf::from).collect();
        self.summarize()
    }

    pub(crate) fn from_dir(&mut self, input_dir: &str) -> PyResult<()> {
        self.input_files = find_maf_files(input_dir);
        self.summarize()
    }

    fn summarize(&self) -> PyResult<()> {
        Summary::new(
            &self.input_files,
            &self.output_path,
            self.output_prefix.as_deref(),
        )
        .with_naming(self.naming)
        .with_ref_species(self.ref_species.clone())
        .with_species(self.species.clone())
        .summarize()
        .map_err(to_py_err)?;
        Ok(())
    }
}
