import pysegul

from os import listdir


def test_sequence_addition(tmp_path):
    input_dir = 'tests/align-data'
    output_path = tmp_path.joinpath('results')
    add = pysegul.SequenceAddition('nexus', 'dna', str(output_path), 'fasta')
    add.input_dir = input_dir
    add.add_to_dir(input_dir, 'nexus')
    assert output_path.exists()
    results = listdir(output_path)
    assert len(results) == 4


def test_sequence_addition_single_file(tmp_path):
    output_file = tmp_path.joinpath('results', 'all.fas')
    add = pysegul.SequenceAddition('nexus', 'dna', str(tmp_path.joinpath('results')))
    add.input_dir = 'tests/align-data'
    add.add_to_file(str(output_file), include_filename=True)
    assert output_file.exists()
