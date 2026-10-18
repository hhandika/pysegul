import pysegul

from os import listdir


def test_align_trim(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    output_path = tmp_path.joinpath('results')
    trim = pysegul.AlignmentTrimming(
        input_format,
        datatype,
        str(output_path),
        output_format,
        )
    trim.input_dir = input_dir
    trim.missing_data(0.5)
    assert output_path.joinpath('trimming_summary.csv').exists()
    results = listdir(output_path.joinpath('trimmed_alignments'))
    assert len(results) == 4
