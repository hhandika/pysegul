import pysegul

from os import listdir

def test_filter(tmp_path):
    input_dir = 'tests/data'
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