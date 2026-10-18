use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul_core::{
    core::align::filter::{AlignmentFiltering as SeqFilter, FilteringParameters},
    helper::{
        finder::{IDs, SeqFileFinder},
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

use crate::common::{SEQ_DATA_TYPE_ERR, SEQ_INPUT_FMT_ERR, SEQ_OUTPUT_FMT_ERR};

macro_rules! filter_aln {
    ($self: ident, $input_files: ident, $parameter: ident) => {
        let mut handle = SeqFilter::new(
            &$self.$input_files,
            &$self.input_fmt,
            &$self.datatype,
            &$self.output_path,
            &$parameter,
        );
        if $self.is_concat {
            let prefix = Path::new(
                $self
                    .prefix
                    .as_ref()
                    .expect("Prefix is required for concatenation"),
            );
            let partition_fmt = $self
                .partition_fmt
                .as_ref()
                .expect("Partition format is required for concatenation")
                .parse::<PartitionFmt>()
                .expect("Invalid partition format");
            handle.set_concat(&$self.output_fmt, &partition_fmt, prefix);
            handle.filter();
        } else {
            handle.filter();
        }
    };
}

#[pyclass]
pub(crate) struct AlignmentFiltering {
    #[pyo3(set)]
    input_files: Vec<PathBuf>,
    input_fmt: InputFmt,
    datatype: DataType,
    output_path: PathBuf,
    output_fmt: OutputFmt,
    is_concat: bool,
    pub prefix: Option<String>,
    // Output partition format
    pub partition_fmt: Option<String>,
}

#[pymethods]
impl AlignmentFiltering {
    #[new]
    #[pyo3(signature = (input_fmt, datatype, output_path, output_fmt, is_concat, prefix=None, partition_fmt=None))]
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        is_concat: bool,
        prefix: Option<String>,
        partition_fmt: Option<String>,
    ) -> Self {
        let input_fmt = input_fmt.parse().expect(SEQ_INPUT_FMT_ERR);
        let datatype = datatype.parse().expect(SEQ_DATA_TYPE_ERR);
        let output_path = PathBuf::from(output_path);
        let output_fmt = output_fmt.parse().expect(SEQ_OUTPUT_FMT_ERR);
        let prefix = prefix;
        let partition_fmt = partition_fmt;

        Self {
            input_files: Vec::new(),
            input_fmt,
            datatype,
            output_path,
            output_fmt,
            is_concat,
            prefix,
            partition_fmt,
        }
    }

    #[setter(input_files)]
    pub(crate) fn set_input_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.into_iter().map(PathBuf::from).collect();
    }

    #[setter(input_dir)]
    pub(crate) fn set_input_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
    }

    pub(crate) fn minimal_taxa(&self, percent_taxa: f64) {
        let taxon_count = IDs::new(&self.input_files, &self.input_fmt, &self.datatype)
            .id_unique()
            .len();
        let min_taxa = self.count_min_tax(percent_taxa, taxon_count);
        let parameter = FilteringParameters::MinTax(min_taxa);
        filter_aln!(self, input_files, parameter);
    }

    // Keep alignments with at least `min_taxa` taxa.
    pub(crate) fn minimal_taxa_count(&self, min_taxa: usize) {
        let parameter = FilteringParameters::MinTax(min_taxa);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn minimal_length(&self, min_length: usize) {
        let parameter = FilteringParameters::MinLen(min_length);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn maximal_length(&self, max_length: usize) {
        let parameter = FilteringParameters::MaxLen(max_length);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn minimal_parsimony_inf(&self, min_parsimony: usize) {
        let parameter = FilteringParameters::MinParsInf(min_parsimony);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn maximal_parsimony_inf(&self, max_parsimony: usize) {
        let parameter = FilteringParameters::MaxParsInf(max_parsimony);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn percent_informative(&self, percent_informative: f64) {
        let parameter = FilteringParameters::PercInf(percent_informative);
        filter_aln!(self, input_files, parameter);
    }

    // Keep alignments whose proportion of missing data (gaps and '?')
    // is at most `max_missing` (0.0 - 1.0).
    pub(crate) fn missing_data(&self, max_missing: f64) {
        let parameter = FilteringParameters::MissingData(max_missing);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn contain_taxa(&self, taxon_id: Vec<String>) {
        let parameter = FilteringParameters::TaxonAll(taxon_id);
        filter_aln!(self, input_files, parameter);
    }

    fn count_min_tax(&self, percent: f64, taxon_count: usize) -> usize {
        (percent * taxon_count as f64).floor() as usize
    }
}
