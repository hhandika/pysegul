import pysegul

from os import listdir


def test_align_unalign(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_path = tmp_path.joinpath('results')
    unalign = pysegul.AlignmentUnalignment(
        input_format,
        datatype,
        str(output_path),
        'fasta',
        )
    unalign.from_dir(input_dir)
    assert output_path.exists()
    results = listdir(output_path)
    assert len(results) == 4
