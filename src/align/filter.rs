use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*};
use segul_core::{
    core::align::filter::{AlignmentFiltering as SeqFilter, FilteringParameters},
    helper::{
        finder::{IDs, SeqFileFinder},
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

use crate::common::{parse, to_paths, SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

use super::PARTITION_FMT_ERR;

/// Filter alignments. Set the input with `input_files` or `input_dir`,
/// then call a filtering method.
///
/// input_fmt: 'auto', 'fasta', 'nexus', or 'phylip'.
/// datatype: 'dna', 'aa', or 'ignore'.
/// output_path: Output directory.
/// output_fmt: 'fasta', 'nexus', 'phylip', or their interleaved forms.
/// is_concat: Concatenate the alignments that pass the filter.
/// prefix: Name of the concatenated alignment. Required with `is_concat`.
/// partition_fmt: Partition format of the concatenated alignment.
///     Required with `is_concat`.
#[pyclass]
pub(crate) struct AlignmentFiltering {
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
    // Prefix and partition format of the concatenated alignment.
    concat: Option<(PathBuf, PartitionFmt)>,
}

#[pymethods]
impl AlignmentFiltering {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt, is_concat=false, prefix=None, partition_fmt=None))]
    fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        is_concat: bool,
        prefix: Option<String>,
        partition_fmt: Option<String>,
    ) -> PyResult<Self> {
        let concat = if is_concat {
            match (prefix, partition_fmt) {
                (Some(prefix), Some(part_fmt)) => {
                    Some((PathBuf::from(prefix), parse(&part_fmt, PARTITION_FMT_ERR)?))
                }
                _ => {
                    return Err(PyValueError::new_err(
                        "Concatenation requires `prefix` and `partition_fmt`.",
                    ))
                }
            }
        } else {
            None
        };
        Ok(Self {
            input_files: Vec::new(),
            input_fmt: parse(input_fmt, SEQ_INPUT_FMT_ERR)?,
            datatype: parse(datatype, SEQ_DATA_TYPE_ERR)?,
            output_path: PathBuf::from(output_path),
            output_fmt: parse(output_fmt, SEQ_OUTPUT_FMT_ERR)?,
            concat,
        })
    }

    /// Input alignment files.
    #[setter(input_files)]
    fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = to_paths(input_files);
    }

    /// Directory of input alignment files.
    #[setter(input_dir)]
    fn set_input_dir(&mut self, input_dir: &str) {
        self.input_files = SeqFileFinder::new(Path::new(input_dir)).find(&self.input_fmt);
    }

    /// Keep alignments with at least `percent_taxa` (0.0 - 1.0) of all taxa.
    fn minimal_taxa(&self, percent_taxa: f64) {
        let taxon_count = IDs::new(&self.input_files, &self.input_fmt, &self.datatype)
            .id_unique()
            .len();
        let min_taxa = (percent_taxa * taxon_count as f64).floor() as usize;
        self.filter(FilteringParameters::MinTax(min_taxa));
    }

    /// Keep alignments with at least `min_taxa` taxa.
    fn minimal_taxa_count(&self, min_taxa: usize) {
        self.filter(FilteringParameters::MinTax(min_taxa));
    }

    /// Keep alignments at least `min_length` sites long.
    fn minimal_length(&self, min_length: usize) {
        self.filter(FilteringParameters::MinLen(min_length));
    }

    /// Keep alignments at most `max_length` sites long.
    fn maximal_length(&self, max_length: usize) {
        self.filter(FilteringParameters::MaxLen(max_length));
    }

    /// Keep alignments with at least `min_parsimony` parsimony informative sites.
    fn minimal_parsimony_inf(&self, min_parsimony: usize) {
        self.filter(FilteringParameters::MinParsInf(min_parsimony));
    }

    /// Keep alignments with at most `max_parsimony` parsimony informative sites.
    fn maximal_parsimony_inf(&self, max_parsimony: usize) {
        self.filter(FilteringParameters::MaxParsInf(max_parsimony));
    }

    /// Keep alignments whose share of parsimony informative sites
    /// is at least `percent_informative` (0.0 - 1.0).
    fn percent_informative(&self, percent_informative: f64) {
        self.filter(FilteringParameters::PercInf(percent_informative));
    }

    /// Keep alignments whose proportion of missing data (gaps and '?')
    /// is at most `max_missing` (0.0 - 1.0).
    fn missing_data(&self, max_missing: f64) {
        self.filter(FilteringParameters::MissingData(max_missing));
    }

    /// Keep alignments that contain all of the given taxa.
    fn contain_taxa(&self, taxon_id: Vec<String>) {
        self.filter(FilteringParameters::TaxonAll(taxon_id));
    }
}

impl AlignmentFiltering {
    fn filter(&self, parameter: FilteringParameters) {
        let mut handle = SeqFilter::new(
            &self.input_files,
            &self.input_fmt,
            &self.datatype,
            &self.output_path,
            &parameter,
        );
        if let Some((prefix, partition_fmt)) = &self.concat {
            handle.set_concat(&self.output_fmt, partition_fmt, prefix);
        }
        handle.filter();
    }
}
