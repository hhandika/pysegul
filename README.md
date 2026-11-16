# PySEGUL

![Test](https://github.com/hhandika/pysegul/workflows/test/badge.svg)
[![PyPI version](https://badge.fury.io/py/pysegul.svg)](https://badge.fury.io/py/pysegul)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pysegul)

PySEGUL is a python bindings of the [SEGUL](https://segul.app) high-performance and memory-efficient genomic tools. It is well-suited for large-scale genomic projects involving thousands of loci, but it is just as capable of handling small Sanger sequences effectively.

Learn more on using PySEGUL in the [documentation](https://www.segul.app/docs/api-usage/python/intro).

## Features

| SEGUL command               | PySEGUL class                                |
| --------------------------- | -------------------------------------------- |
| `segul align concat`        | `AlignmentConcatenation`                     |
| `segul align convert`       | `AlignmentConversion`                        |
| `segul align distance`      | `AlignmentDistance` _(beta)_                 |
| `segul align filter`        | `AlignmentFiltering`                         |
| `segul align split`         | `AlignmentSplitting`                         |
| `segul align summary`       | `AlignmentSummary`                           |
| `segul align trim`          | `AlignmentTrimming` _(beta)_                 |
| `segul align unalign`       | `AlignmentUnalignment` _(beta)_              |
| `segul partition`           | `PartitionConversion`                        |
| `segul read summary`        | `ReadSummary`                                |
| `segul contig summary`      | `ContigSummary`                              |
| `segul maf convert`         | `MafConversion` _(beta)_                     |
| `segul maf summary`         | `MafSummary` _(beta)_                        |
| `segul sequence add`        | `SequenceAddition` _(beta)_                  |
| `segul sequence extract`    | `SequenceExtraction`                         |
| `segul sequence filter`     | `SequenceFiltering` _(beta)_                 |
| `segul sequence id`         | `IDExtraction`                               |
| `segul sequence remove`     | `SequenceRemoval`                            |
| `segul sequence rename`     | `SequenceRenaming` _(beta)_                  |
| `segul sequence translate`  | `SequenceTranslation`                        |
| `segul sequence transpose`  | `SequenceTranspose` _(beta)_                 |
| `--plot` and report viewers | `PlotOptions` and `load_*_report()` _(beta)_ |

Features marked _(beta)_ are new and may still change. If you use PySEGUL in a publication, always report the exact version you used.

Summary classes (`AlignmentSummary`, `AlignmentDistance`, `ReadSummary`, `ContigSummary`, `MafSummary`) take a `plot` option to write charts, like the CLI's `--plot`. Use `load_align_report()`, `load_contig_report()`, `load_distance_report()`, or `load_read_report()` to read their JSON reports as dicts.

```python
import pysegul

summary = pysegul.AlignmentSummary(
    'nexus', 'dna', 'results/summary', plot=pysegul.PlotOptions(format='svg')
    )
outputs = summary.from_dir('alignments')
report = pysegul.load_align_report(outputs['report'])
print(outputs['plots'])
```

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
