import pysegul

from os import listdir

def test_align_filter(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'nexus'
    is_concat = False
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    filter = pysegul.AlignmentFiltering(
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        is_concat
        )
    filter.input_dir = input_dir
    filter.minimal_length(8)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 1


def test_align_filter_max_length(tmp_path):
    output_path = tmp_path.joinpath('results')
    filter = pysegul.AlignmentFiltering('nexus', 'dna', str(output_path), 'nexus', False)
    filter.input_dir = 'tests/align-data'
    filter.maximal_length(6)
    assert len(listdir(output_path)) == 3


def test_align_filter_min_taxa_count(tmp_path):
    output_path = tmp_path.joinpath('results')
    filter = pysegul.AlignmentFiltering('nexus', 'dna', str(output_path), 'nexus', False)
    filter.input_dir = 'tests/align-data'
    filter.minimal_taxa_count(2)
    assert len(listdir(output_path)) == 3
