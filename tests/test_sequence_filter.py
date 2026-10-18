import pysegul

from os import listdir


def test_sequence_filter(tmp_path):
    input_dir = 'tests/align-data'
    output_path = tmp_path.joinpath('results')
    filter = pysegul.SequenceFiltering('nexus', 'dna', str(output_path), 'fasta')
    filter.input_dir = input_dir
    filter.minimal_length(7)
    assert output_path.exists()
    results = listdir(output_path)
    assert len(results) == 1
