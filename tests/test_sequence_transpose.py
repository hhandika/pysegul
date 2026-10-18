import pysegul

from os import listdir


def test_sequence_transpose(tmp_path):
    input_dir = 'tests/align-data'
    output_path = tmp_path.joinpath('results')
    transpose = pysegul.SequenceTranspose('nexus', 'dna', str(output_path), 'fasta')
    transpose.from_dir(input_dir)
    assert sorted(listdir(output_path)) == ['ABCD', 'ABCE', 'ABCF']
    assert output_path.joinpath('ABCD', 'ABCD.fas').exists()
