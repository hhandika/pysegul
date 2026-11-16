import pysegul
import pytest

import os

def test_sequence_extract(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    extract = pysegul.SequenceExtraction(
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        )
    extract.input_dir = input_dir
    extract.extract_regex("(?i)^(abce)")
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = os.listdir(output_path)
    assert len(results) == 2
    # Clean up output directory before the next test
    for i in results:
        os.remove(output_path.joinpath(i))
    extract.extract_id_list(['ABCD'])
    assert output_path.exists()
    results = os.listdir(output_path)
    assert len(results) == 4

def test_sequence_extract_id_file(tmp_path):
    output_path = tmp_path.joinpath('results')
    extract = pysegul.SequenceExtraction('nexus', 'dna', str(output_path), 'fasta')
    extract.input_dir = 'tests/align-data'
    extract.extract_id_file('tests/id-data/ids.txt')
    assert len(os.listdir(output_path)) == 4


def test_sequence_extract_missing_id_file(tmp_path):
    extract = pysegul.SequenceExtraction('nexus', 'dna', str(tmp_path), 'fasta')
    extract.input_dir = 'tests/align-data'
    with pytest.raises(FileNotFoundError):
        extract.extract_id_file('tests/id-data/missing.txt')
