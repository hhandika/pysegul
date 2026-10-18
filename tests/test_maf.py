import pysegul

from os import listdir


def test_maf_convert(tmp_path):
    output_path = tmp_path.joinpath('results')
    convert = pysegul.MafConversion(str(output_path), output_fmt='fasta')
    convert.from_dir('tests/maf-data')
    assert output_path.exists()
    results = listdir(output_path)
    assert len(results) == 3


def test_maf_convert_bed(tmp_path):
    output_path = tmp_path.joinpath('results')
    convert = pysegul.MafConversion(
        str(output_path),
        output_fmt='nexus',
        bed='tests/maf-data/simple_regions.bed',
        species=['hg18', 'mm4'],
        output_prefix='loci',
        )
    convert.from_files(['tests/maf-data/simple.maf'])
    assert output_path.joinpath('loci.csv').exists()
    results = [f for f in listdir(output_path) if f.endswith('.nex')]
    assert len(results) == 4


def test_maf_summary(tmp_path):
    output_path = tmp_path.joinpath('results')
    summary = pysegul.MafSummary(str(output_path), output_prefix='maf')
    summary.from_dir('tests/maf-data')
    assert output_path.joinpath('maf_per_species.csv').exists()
    assert output_path.joinpath('maf_per_file.csv').exists()
