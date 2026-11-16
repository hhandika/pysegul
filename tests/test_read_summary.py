import pysegul
import pytest

from os import listdir

def test_read_summary(tmp_path):
    input_dir = 'tests/raw-data'
    input_format = 'auto'
    summary_mode = 'default'
    prefix = 'read_summary'
    output_path = tmp_path.joinpath('results')
    output_dir = str(output_path)
    summary = pysegul.ReadSummary(
        input_format,  
        summary_mode,
        output_dir,
        prefix
        )
    summary.from_dir(input_dir)
    assert output_path.exists()
    # Check if the output directory contains the expected files
    results = listdir(output_path)
    assert len(results) == 1

def test_read_summary_complete(tmp_path):
    output_path = tmp_path.joinpath('results')
    summary = pysegul.ReadSummary(
        'auto',
        'complete',
        str(output_path),
        'read_summary',
        phred='33',
        dup_sketch_size=1000,
        )
    outputs = summary.from_dir('tests/raw-data')
    assert outputs['report'] is not None
    report = pysegul.load_read_report(outputs['report'])
    assert len(report['samples']) == 4
    sample = report['samples'][0]
    assert sample['qc_status'] in ('Pass', 'Warn', 'Fail')
    assert sample['qc'][0]['name'] == 'Q30 bases'


def test_read_summary_invalid_phred(tmp_path):
    with pytest.raises(ValueError):
        pysegul.ReadSummary('auto', 'default', str(tmp_path), phred='50')
