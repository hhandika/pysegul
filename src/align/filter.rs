use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use segul::{
    handler::align::filter::{Params, SeqFilter},
    helper::{
        files,
        finder::{IDs, SeqFileFinder},
        types::{DataType, InputFmt, OutputFmt, PartitionFmt},
    },
};

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
            let output_path =
                files::create_output_fname(&$self.output_path, prefix, &$self.output_fmt);
            handle.set_concat(&output_path, &$self.output_fmt, &partition_fmt);
            handle.filter_aln();
        } else {
            handle.filter_aln();
        }
    };
}

#[pyclass]
pub struct AlignmentFiltering {
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
    pub(crate) fn new(
        input_fmt: &str,
        datatype: &str,
        output_path: &str,
        output_fmt: &str,
        is_concat: bool,
        prefix: Option<String>,
        partition_fmt: Option<String>,
    ) -> Self {
        let input_fmt = input_fmt.parse().expect("Invalid input format");
        let datatype = datatype.parse().expect("Invalid data type");
        let output_path = PathBuf::from(output_path);
        let output_fmt = output_fmt.parse().expect("Invalid output format");
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

    #[setter]
    pub(crate) fn input_from_files(&mut self, input_files: Vec<String>) {
        self.input_files = input_files.into_iter().map(PathBuf::from).collect();
    }

    #[setter]
    pub(crate) fn input_from_dir(&mut self, input_dir: &str) {
        let input_dir = Path::new(input_dir);
        self.input_files = SeqFileFinder::new(input_dir).find(&self.input_fmt);
    }

    pub(crate) fn minimal_taxa(&self, percent_taxa: f64) {
        let taxon_count = IDs::new(&self.input_files, &self.input_fmt, &self.datatype)
            .id_unique()
            .len();
        let min_taxa = self.count_min_tax(percent_taxa, taxon_count);
        let parameter = Params::MinTax(min_taxa);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn minimal_length(&self, min_length: usize) {
        let parameter = Params::AlnLen(min_length);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn minimal_parsimony_inf(&self, min_parsimony: usize) {
        let parameter = Params::ParsInf(min_parsimony);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn percent_informative(&self, percent_informative: f64) {
        let parameter = Params::PercInf(percent_informative);
        filter_aln!(self, input_files, parameter);
    }

    pub(crate) fn contain_taxa(&self, taxon_id: Vec<String>) {
        let parameter = Params::TaxonAll(taxon_id);
        filter_aln!(self, input_files, parameter);
    }

    fn count_min_tax(&self, percent: f64, taxon_count: usize) -> usize {
        (percent * taxon_count as f64).floor() as usize
    }
}
