import pysegul

from os import listdir

def test_align_summary(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    completeness_interval = 5
    prefix = 'concatenated'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    concat = pysegul.AlignmentSummary(
        input_format,  
        datatype, 
        output_dir,
        completeness_interval,
        prefix
        )
    concat.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 3


def test_align_summary_per_locus(tmp_path):
    output_path = tmp_path.joinpath('results')
    summary = pysegul.AlignmentSummary(
        'nexus', 'dna', str(output_path), 5, per_locus=True
        )
    summary.from_dir('tests/align-data')
    assert len(listdir(output_path)) == 4
