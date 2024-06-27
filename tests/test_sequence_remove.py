import pysegul

import os

def test_sequence_extract(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    remove = pysegul.SequenceRemoval(
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        )
    remove.input_dir = input_dir
    remove.remove_regex("(?i)^(abce)")
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = os.listdir(output_path)
    assert len(results) == 4
    for i in results:
        os.remove(output_path.joinpath(i))
    remove.remove_id_list(['ABCD'])
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = os.listdir(output_path)
    assert len(results) == 3