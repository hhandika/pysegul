import pysegul

from os import listdir

def test_contig_summary(tmp_path):
    input_dir = 'tests/contig-data'
    input_format = 'auto'
    prefix = 'contig_summary'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    summary = pysegul.ContigSummary(
        input_format,  
        output_dir,
        prefix
        )
    summary.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 1