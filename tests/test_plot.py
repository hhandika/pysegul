import pysegul
import pytest

from pathlib import Path

# SVG charts need no fonts, so these run on any machine.
SVG = pysegul.PlotOptions(format='svg')


def assert_plots(outputs):
    assert len(outputs['plots']) > 0
    for path in outputs['plots']:
        assert Path(path).exists()
        assert path.endswith('.svg')


def test_align_summary_plot(tmp_path):
    summary = pysegul.AlignmentSummary('nexus', 'dna', str(tmp_path), plot=SVG)
    outputs = summary.from_dir('tests/align-data')
    assert_plots(outputs)
    assert Path(outputs['plots'][0]).parent == tmp_path.joinpath('plots')


def test_align_summary_per_locus_plot(tmp_path):
    with pytest.raises(ValueError):
        pysegul.AlignmentSummary('nexus', 'dna', str(tmp_path), per_locus=True, plot=SVG)


def test_contig_summary_plot(tmp_path):
    summary = pysegul.ContigSummary('auto', str(tmp_path), plot=SVG)
    assert_plots(summary.from_dir('tests/contig-data'))


def test_read_summary_plot(tmp_path):
    summary = pysegul.ReadSummary('auto', 'complete', str(tmp_path), plot=SVG)
    assert_plots(summary.from_dir('tests/raw-data'))


def test_distance_plot(tmp_path):
    distance = pysegul.AlignmentDistance('nexus', 'dna', str(tmp_path), plot=SVG)
    assert_plots(distance.from_dir('tests/align-data'))


def test_maf_summary_plot(tmp_path):
    summary = pysegul.MafSummary(str(tmp_path), plot=SVG)
    assert_plots(summary.from_dir('tests/maf-data'))


def test_plot_options_custom():
    pysegul.PlotOptions(
        format='png', palette='okabe-ito', colormap='cividis', cvd='deuteranopia'
        )


@pytest.mark.parametrize('options', [
    {'format': 'jpg'},
    {'palette': 'rainbow'},
    {'colormap': 'jet'},
    {'cvd': 'blue'},
])
def test_plot_options_invalid(options):
    with pytest.raises(ValueError):
        pysegul.PlotOptions(**options)
