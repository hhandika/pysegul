import pysegul

from os import listdir

def test_id_extraction(tmp_path):
    input_dir = 'tests/data'
    input_format = 'nexus'
    datatype = 'dna'
    map_id = False
    prefix = 'concatenated'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    concat = pysegul.IDExtraction(
        input_format,  
        datatype, 
        output_dir, 
        map_id, 
        prefix
        )
    concat.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 1