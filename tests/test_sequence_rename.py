import pysegul


def test_sequence_rename(tmp_path):
    input_dir = 'tests/align-data'
    output_path = tmp_path.joinpath('results')
    rename = pysegul.SequenceRenaming('nexus', 'dna', str(output_path), 'fasta')
    rename.input_dir = input_dir
    rename.replace_string('ABCD', 'XYZ')
    with open(output_path.joinpath('gene_1.fas')) as f:
        assert '>XYZ' in f.read()


def test_sequence_rename_regex(tmp_path):
    output_path = tmp_path.joinpath('results')
    rename = pysegul.SequenceRenaming('nexus', 'dna', str(output_path), 'fasta')
    rename.input_files = ['tests/align-data/gene_2.nex']
    rename.replace_regex('^AB', 'Q')
    with open(output_path.joinpath('gene_2.fas')) as f:
        content = f.read()
    assert '>QCD' in content
    assert '>QCF' in content


def test_sequence_rename_dry_run(tmp_path):
    output_path = tmp_path.joinpath('results')
    rename = pysegul.SequenceRenaming(
        'nexus', 'dna', str(output_path), 'fasta', dry_run=True
        )
    rename.input_dir = 'tests/align-data'
    rename.remove_string('AB')
    assert not output_path.exists()
