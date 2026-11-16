"""Type stubs for pysegul, Python bindings for SEGUL."""

from typing import Any, Dict, List, Optional, Tuple, final

# Output paths returned by the summary classes. Missing outputs are None.
# Summary classes also return 'plots', a list of chart paths.
_OutputPaths = Dict[str, Any]

def alphabet() -> str: ...

# Reports

def load_align_report(path: str) -> Dict[str, Any]: ...
def load_contig_report(path: str) -> Dict[str, Any]: ...
def load_distance_report(path: str) -> Dict[str, Any]: ...
def load_read_report(path: str) -> Dict[str, Any]: ...

# Plots

@final
class PlotOptions:
    def __new__(
        cls,
        format: str = "png",
        font: Optional[str] = None,
        palette: str = "dark2",
        colormap: str = "viridis",
        cvd: Optional[str] = None,
    ) -> "PlotOptions": ...

# Alignments

@final
class AlignmentConcatenation:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_dir: str,
        output_fmt: str,
        partition_fmt: str,
        output_prefix: Optional[str] = None,
    ) -> "AlignmentConcatenation": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...

@final
class AlignmentConversion:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_fmt: str,
        sort_sequence: bool = False,
    ) -> "AlignmentConversion": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...

@final
class AlignmentDistance:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        model: str = "p",
        triangle: str = "full",
        diagonal: Optional[str] = None,
        long_format: bool = False,
        output_prefix: Optional[str] = None,
        plot: Optional[PlotOptions] = None,
    ) -> "AlignmentDistance": ...
    def group_by_delimiter(self, delimiter: str, fields: int) -> None: ...
    def group_by_regex(self, pattern: str) -> None: ...
    def group_by_file(self, path: str) -> None: ...
    def group_auto(self) -> None: ...
    def from_files(self, input_files: List[str]) -> _OutputPaths: ...
    def from_dir(self, input_dir: str) -> _OutputPaths: ...

@final
class AlignmentFiltering:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_fmt: str,
        is_concat: bool = False,
        prefix: Optional[str] = None,
        partition_fmt: Optional[str] = None,
    ) -> "AlignmentFiltering": ...
    def minimal_taxa(self, percent_taxa: float) -> None: ...
    def minimal_taxa_count(self, min_taxa: int) -> None: ...
    def minimal_length(self, min_length: int) -> None: ...
    def maximal_length(self, max_length: int) -> None: ...
    def minimal_parsimony_inf(self, min_parsimony: int) -> None: ...
    def maximal_parsimony_inf(self, max_parsimony: int) -> None: ...
    def percent_informative(self, percent_informative: float) -> None: ...
    def missing_data(self, max_missing: float) -> None: ...
    def contain_taxa(self, taxon_id: List[str]) -> None: ...

@final
class AlignmentSplitting:
    def __new__(
        cls,
        input_path: str,
        input_fmt: str,
        datatype: str,
        output_dir: str,
        output_fmt: str,
        partition_fmt: str,
        check_partition: bool = True,
        input_partition: Optional[str] = None,
        output_prefix: Optional[str] = None,
    ) -> "AlignmentSplitting": ...
    def split(self) -> None: ...

@final
class AlignmentSummary:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        summary_interval: int = 5,
        output_prefix: Optional[str] = None,
        per_locus: bool = False,
        plot: Optional[PlotOptions] = None,
    ) -> "AlignmentSummary": ...
    def from_files(self, input_files: List[str]) -> _OutputPaths: ...
    def from_dir(self, input_dir: str) -> _OutputPaths: ...

@final
class AlignmentTrimming:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls, input_fmt: str, datatype: str, output_path: str, output_fmt: str
    ) -> "AlignmentTrimming": ...
    def missing_data(self, max_missing: float) -> None: ...
    def parsimony_inf(self, threshold: int) -> None: ...

@final
class AlignmentUnalignment:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_fmt: str = "fasta",
    ) -> "AlignmentUnalignment": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...

@final
class PartitionConversion:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_dir: str,
        output_fmt: str,
        check_partition: bool = True,
    ) -> "PartitionConversion": ...
    def from_files(self, input_files: List[str]) -> None: ...

# Genomics

@final
class ReadSummary:
    def __new__(
        cls,
        input_fmt: str,
        mode: str,
        output_path: str,
        output_prefix: Optional[str] = None,
        phred: str = "auto",
        dup_sketch_size: int = 100000,
        plot: Optional[PlotOptions] = None,
    ) -> "ReadSummary": ...
    def from_files(self, input_files: List[str]) -> _OutputPaths: ...
    def from_dir(self, input_dir: str) -> _OutputPaths: ...

@final
class ContigSummary:
    def __new__(
        cls,
        input_fmt: str,
        output_dir: str,
        output_prefix: Optional[str] = None,
        plot: Optional[PlotOptions] = None,
    ) -> "ContigSummary": ...
    def from_files(self, input_files: List[str]) -> _OutputPaths: ...
    def from_dir(self, input_dir: str) -> _OutputPaths: ...

@final
class MafConversion:
    def __new__(
        cls,
        output_path: str,
        output_fmt: str = "fasta",
        group_by: str = "block",
        bed: Optional[str] = None,
        clip: bool = True,
        strand_aware: bool = False,
        full_source_name: bool = False,
        ref_species: Optional[str] = None,
        species: Optional[List[str]] = None,
        min_block_len: Optional[int] = None,
        min_species: Optional[int] = None,
        min_score: Optional[float] = None,
        write_unmatched: bool = False,
        output_prefix: Optional[str] = None,
    ) -> "MafConversion": ...
    def from_files(self, input_files: List[str]) -> Dict[str, int]: ...
    def from_dir(self, input_dir: str) -> Dict[str, int]: ...

@final
class MafSummary:
    def __new__(
        cls,
        output_path: str,
        output_prefix: Optional[str] = None,
        full_source_name: bool = False,
        ref_species: Optional[str] = None,
        species: Optional[List[str]] = None,
        plot: Optional[PlotOptions] = None,
    ) -> "MafSummary": ...
    def from_files(self, input_files: List[str]) -> _OutputPaths: ...
    def from_dir(self, input_dir: str) -> _OutputPaths: ...

# Sequences

@final
class IDExtraction:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        map_id: bool = False,
        output_prefix: Optional[str] = None,
    ) -> "IDExtraction": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...

@final
class SequenceAddition:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_fmt: str = "fasta",
        added_only: bool = False,
    ) -> "SequenceAddition": ...
    def add_to_files(self, dest_files: List[str], dest_fmt: str = "auto") -> None: ...
    def add_to_dir(self, dest_dir: str, dest_fmt: str = "auto") -> None: ...
    def add_to_file(self, dest_file: str, include_filename: bool = False) -> None: ...

@final
class SequenceExtraction:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls, input_fmt: str, datatype: str, output_path: str, output_format: str
    ) -> "SequenceExtraction": ...
    def extract_regex(self, regex: str) -> None: ...
    def extract_id_list(self, list: List[str]) -> None: ...
    def extract_id_file(self, path: str) -> None: ...

@final
class SequenceFiltering:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls, input_fmt: str, datatype: str, output_path: str, output_fmt: str
    ) -> "SequenceFiltering": ...
    def minimal_length(self, min_length: int) -> None: ...
    def maximal_length(self, max_length: int) -> None: ...
    def maximal_gap(self, max_gap: float) -> None: ...

@final
class SequenceRemoval:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls, input_fmt: str, datatype: str, output_path: str, output_format: str
    ) -> "SequenceRemoval": ...
    def remove_regex(self, regex: str) -> None: ...
    def remove_id_list(self, id: List[str]) -> None: ...

@final
class SequenceRenaming:
    input_files: List[str]
    input_dir: str
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_format: str,
        dry_run: bool = False,
    ) -> "SequenceRenaming": ...
    def replace_id(self, names: List[Tuple[str, str]]) -> None: ...
    def replace_id_from_file(self, path: str) -> None: ...
    def remove_string(self, input_str: str) -> None: ...
    def remove_regex(self, regex: str, all: bool = False) -> None: ...
    def replace_string(self, from_: str, /, to: str) -> None: ...
    def replace_regex(self, regex: str, to: str, all: bool = False) -> None: ...

@final
class SequenceTranslation:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_format: str,
        table: str = "1",
        reading_frame: Optional[int] = None,
    ) -> "SequenceTranslation": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...

@final
class SequenceTranspose:
    def __new__(
        cls,
        input_fmt: str,
        datatype: str,
        output_path: str,
        output_fmt: str = "fasta",
    ) -> "SequenceTranspose": ...
    def from_files(self, input_files: List[str]) -> None: ...
    def from_dir(self, input_dir: str) -> None: ...
