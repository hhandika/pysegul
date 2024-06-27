import pysegul

from os import listdir

def test_partition_convert(tmp_path):
    input_paths = [
        'tests/partition-data/partition_codon.txt', 
        'tests/partition-data/partition.txt'
        ]
    input_format = 'raxml'
    output_format = 'nexus'
    datatype = 'dna'
    check_partition = True
    output_dir = str(tmp_path.joinpath('results'))
    convert = pysegul.PartitionConversion(
        input_format,
        datatype,
        output_dir,
        output_format,
        check_partition
        )
    convert.from_files(input_paths)
    assert tmp_path.joinpath('results').exists()
    # Check if the output directory contains the expected files
    results = listdir(output_dir)
    assert len(results) == 2
    assert 'partition_codon_partition.nex' in results