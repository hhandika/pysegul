import pysegul
import pytest


def test_invalid_input_format(tmp_path):
    with pytest.raises(ValueError, match='input format'):
        pysegul.AlignmentConversion('fastq', 'dna', str(tmp_path), 'nexus')


def test_invalid_datatype(tmp_path):
    with pytest.raises(ValueError, match='data type'):
        pysegul.AlignmentConcatenation('nexus', 'rna', str(tmp_path), 'nexus', 'nexus')


def test_invalid_output_format(tmp_path):
    with pytest.raises(ValueError, match='output format'):
        pysegul.AlignmentSplitting(
            'tests/split-data/concat.nex', 'nexus', 'dna', str(tmp_path), 'csv', 'charset'
            )


def test_invalid_partition_format(tmp_path):
    with pytest.raises(ValueError, match='partition format'):
        pysegul.PartitionConversion('nexus', 'dna', str(tmp_path), 'iqtree')


def test_invalid_fasta_only_format(tmp_path):
    with pytest.raises(ValueError):
        pysegul.AlignmentUnalignment('nexus', 'dna', str(tmp_path), 'nexus')


def test_filter_concat_requires_prefix(tmp_path):
    with pytest.raises(ValueError):
        pysegul.AlignmentFiltering('nexus', 'dna', str(tmp_path), 'nexus', True)
