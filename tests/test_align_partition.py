import pysegul

def test_partition_convert(tmp_path):
    input_paths = [
        'tests\partition-data\partition_codon.txt', 
        'tests\partition-data\partition.txt'
        ]
    input_format = 'raxml'
    output_format = 'nexus'
    datatype = 'dna'
    check_partition = True
    output_dir = str(tmp_path.joinpath('results'))
    convert = pysegul.PartitionConvert(
        input_format,
        datatype,
        output_dir,
        output_format,
        check_partition
        )
    convert.from_files(input_paths)