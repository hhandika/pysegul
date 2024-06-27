import pysegul

from os import listdir

def test_sequence_translate(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    table = '1'
    reading_frame = 1
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    translate = pysegul.SequenceTranslation(
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        table,
        reading_frame
        )
    translate.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_dir)
    assert len(results) == 4
    