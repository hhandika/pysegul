import pysegul
import pytest

from os import listdir


def test_align_distance(tmp_path):
    input_dir = 'tests/align-data'
    input_format = 'nexus'
    datatype = 'dna'
    output_path = tmp_path.joinpath('results')
    distance = pysegul.AlignmentDistance(
        input_format,
        datatype,
        str(output_path),
        model='k2p',
        triangle='lower',
        )
    distance.from_dir(input_dir)
    assert output_path.joinpath('distance_summary.csv').exists()
    results = listdir(output_path.joinpath('per_locus'))
    assert len(results) == 4


def test_align_distance_group(tmp_path):
    input_dir = 'tests/align-data'
    output_path = tmp_path.joinpath('results')
    distance = pysegul.AlignmentDistance(
        'nexus',
        'dna',
        str(output_path),
        long_format=True,
        )
    distance.group_by_delimiter('C', 1)
    distance.from_dir(input_dir)
    assert output_path.joinpath('distance_group_mean.csv').exists()
    assert output_path.joinpath('distance_pairwise_long.csv').exists()


def test_align_distance_invalid_model(tmp_path):
    with pytest.raises(ValueError):
        pysegul.AlignmentDistance('nexus', 'dna', str(tmp_path), model='poisson')


def test_align_distance_group_max_diagonal(tmp_path):
    output_path = tmp_path.joinpath('results')
    distance = pysegul.AlignmentDistance(
        'nexus',
        'dna',
        str(output_path),
        diagonal='max',
        )
    distance.group_by_delimiter('C', 1)
    distance.from_dir('tests/align-data')
    assert output_path.joinpath('distance_group_mean.csv').exists()


def test_align_distance_group_diagonal_requires_grouping(tmp_path):
    distance = pysegul.AlignmentDistance(
        'nexus', 'dna', str(tmp_path), diagonal='mean'
        )
    with pytest.raises(ValueError):
        distance.from_dir('tests/align-data')
