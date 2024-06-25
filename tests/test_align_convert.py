import pysegul

from os import listdir

def test_convert(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    sort_sequences = True
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    convert = pysegul.AlignmentConversion(
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        sort_sequences 
        )
    convert.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 4