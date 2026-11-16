import pysegul
import pytest


def test_align_report(tmp_path):
    summary = pysegul.AlignmentSummary('nexus', 'dna', str(tmp_path))
    outputs = summary.from_dir('tests/align-data')
    report = pysegul.load_align_report(outputs['report'])
    assert report['datatype'] == 'dna'
    assert 'summary' in report


def test_contig_report(tmp_path):
    summary = pysegul.ContigSummary('auto', str(tmp_path))
    outputs = summary.from_dir('tests/contig-data')
    report = pysegul.load_contig_report(outputs['report'])
    assert isinstance(report, dict)


def test_distance_report(tmp_path):
    distance = pysegul.AlignmentDistance('nexus', 'dna', str(tmp_path))
    outputs = distance.from_dir('tests/align-data')
    report = pysegul.load_distance_report(outputs['report'])
    assert isinstance(report, dict)


def test_missing_report(tmp_path):
    with pytest.raises(ValueError):
        pysegul.load_align_report(str(tmp_path.joinpath('missing.json')))
