# PySEGUL

![Test](https://github.com/hhandika/pysegul/workflows/ci/badge.svg)
[![PyPI version](https://badge.fury.io/py/pysegul.svg)](https://badge.fury.io/py/pysegul)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pysegul)

PySEGUL is a python bindings of the [SEGUL](https://segul.app) high-performance and memory-efficient phylogenomic tools. It is well-suited for large-scale phylogenomic projects involving thousands of loci, but it is just as capable of handling small Sanger sequences effectively.

Learn more on using PySEGUL in the [documentation](https://www.segul.app/docs/api-usage/python/intro).

## Features

| SEGUL command | PySEGUL class |
| --- | --- |
| `segul align concat` | `AlignmentConcatenation` |
| `segul align convert` | `AlignmentConversion` |
| `segul align distance` | `AlignmentDistance` *(upcoming)* |
| `segul align filter` | `AlignmentFiltering` |
| `segul align split` | `AlignmentSplitting` |
| `segul align summary` | `AlignmentSummary` |
| `segul align trim` | `AlignmentTrimming` *(upcoming)* |
| `segul align unalign` | `AlignmentUnalignment` *(upcoming)* |
| `segul partition` | `PartitionConversion` |
| `segul read summary` | `ReadSummary` |
| `segul contig summary` | `ContigSummary` |
| `segul maf convert` | `MafConversion` *(upcoming)* |
| `segul maf summary` | `MafSummary` *(upcoming)* |
| `segul sequence add` | `SequenceAddition` *(upcoming)* |
| `segul sequence extract` | `SequenceExtraction` |
| `segul sequence filter` | `SequenceFiltering` *(upcoming)* |
| `segul sequence id` | `IDExtraction` |
| `segul sequence remove` | `SequenceRemoval` |
| `segul sequence rename` | `SequenceRenaming` *(upcoming)* |
| `segul sequence translate` | `SequenceTranslation` |
| `segul sequence transpose` | `SequenceTranspose` *(upcoming)* |

Features marked *(upcoming)* are not yet published on PyPI.

## Quick Start

```bash
pip install pysegul
```

To concatenate alignments:

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
