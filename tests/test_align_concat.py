import pysegul

from os import listdir

input_format = 'nexus'
datatype = 'dna'
output_format = 'fasta'
partition_format = 'raxml'
prefix = 'concatenated'

def test_concat_alignments(tmp_path):
    input_dir = 'tests/data'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    concat = pysegul.AlignmentConcatenation(
        input_format,  
        datatype, 
        output_dir, 
        output_format, 
        partition_format, 
        prefix
        )
    concat.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 2

def test_concat_alignments_from_list(tmp_path):
    files: list = ['tests/data/gene_1.nex', 'tests/data/gene_2.nex']
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    concat = pysegul.AlignmentConcatenation(
        input_format,  
        datatype, 
        output_dir, 
        output_format, 
        partition_format, 
        prefix
        )
    concat.from_files(files)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 2