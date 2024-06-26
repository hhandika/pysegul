import pysegul


from os import listdir

def test_align_split(tmp_path):
    input_alignment = 'tests/split-data/split-data.nex'
    input_format = 'nexus'
    datatype = 'dna'
    partition_format = 'nexus'
    output_format = 'fasta'
    check_partition = True
    input_partition = 'tests/split-data/split-data_partition.nex'
    output_dir = str(tmp_path.joinpath('results'))
    split = pysegul.AlignmentSplitting(
        input_alignment,
        input_format,  
        datatype, 
        output_dir, 
        output_format,
        partition_format,
        check_partition,
        input_partition = input_partition
        )
    split.split()
    assert tmp_path.joinpath('results').exists()
    # Check if the output directory contains the expected files
    results = listdir(output_dir)
    assert len(results) == 4