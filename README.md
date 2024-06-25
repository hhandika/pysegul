# pysegul

![Test](https://github.com/hhandika/pysegul/workflows/ci/badge.svg)
[![PyPI version](https://badge.fury.io/py/pysegul.svg)](https://badge.fury.io/py/pysegul)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pysegul)

A python bindings of the [SEGUL](https://segul.app) high-performance and memory-efficient phylogenomic tools. It's fast and memory-efficient. It can handle large datasets with ease.

Usage: [Docs](https://www.segul.app/docs/api-usage/python-api)

## Quick Start

```bash
pip install pysegul
```

To concatenate alignments, you can use the following script:

```python
import pysegul

def concat_alignments():
    input_dir = 'tests/concat'
    input_format = 'nexus'
    datatype = 'dna'
    output_format = 'fasta'
    partition_format = 'raxml'
    prefix = 'concatenated'
    output_dir = 'results/concat'
    concat = pysegul.AlignmentConcatenation(
        input_format,  
        datatype, 
        output_dir, 
        output_format, 
        partition_format, 
        prefix
        )
    concat.from_dir(input_dir)

if __name__ == '__main__':
    concat_alignments()
```
