# PySEGUL

![Test](https://github.com/hhandika/pysegul/workflows/ci/badge.svg)
[![PyPI version](https://badge.fury.io/py/pysegul.svg)](https://badge.fury.io/py/pysegul)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pysegul)

PySEGUL is a python bindings of the [SEGUL](https://segul.app) high-performance and memory-efficient phylogenomic tools. It is well-suited for large-scale phylogenomic projects involving thousands of loci, but it is just as capable of handling small Sanger sequences effectively.

Learn more on using PySEGUL in the [documentation](https://www.segul.app/docs/api-usage/python/intro).

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

## Features

PySEGUL covers the same features as the SEGUL command line app.

Features marked *(upcoming)* are not yet published on PyPI. They are available when you build PySEGUL from source.

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

### Common options

- Input format (`input_fmt`): `'fasta'`, `'nexus'`, `'phylip'`, or `'auto'`.
- Data type (`datatype`): `'dna'`, `'aa'`, or `'ignore'`.
- Output format (`output_fmt`): `'fasta'`, `'nexus'`, `'phylip'`, `'fasta-int'`, `'nexus-int'`, or `'phylip-int'` (the `-int` formats are interleaved).
- Partition format (`partition_fmt`): `'charset'`, `'charset-codon'`, `'nexus'`, `'nexus-codon'`, `'raxml'`, or `'raxml-codon'`.

Classes take input in one of two ways:

- Classes that do one task run it with `from_files(files)` or `from_dir(input_dir)`.
- Classes with several tasks take the input first, then run a task method:

```python
remove = pysegul.SequenceRemoval('nexus', 'dna', 'results/remove', 'fasta')
remove.input_dir = 'alignments'          # or: remove.input_files = ['a.nex', 'b.nex']
remove.remove_id_list(['sample_1', 'sample_2'])
```

Invalid formats or options raise an exception.

## Alignment

### AlignmentConcatenation

```python
pysegul.AlignmentConcatenation(input_fmt, datatype, output_dir, output_fmt, partition_fmt, output_prefix=None)
```

Run with `from_files(files)` or `from_dir(input_dir)`.

### AlignmentConversion

```python
pysegul.AlignmentConversion(input_fmt, datatype, output_path, output_fmt, sort_sequence)
```

Run with `from_files(files)` or `from_dir(input_dir)`. Set `sort_sequence=True` to sort sequences by ID.

### AlignmentDistance *(upcoming)*

Compute pairwise genetic distances for each alignment.

```python
pysegul.AlignmentDistance(
    input_fmt,
    datatype,
    output_path,
    model='p',
    triangle='full',
    diagonal='zero',
    group_diagonal='within',
    long_format=False,
    output_prefix=None,
)
```

- `model`: `'p'` (uncorrected), `'jc69'`, `'k2p'`, or `'tn93'` for DNA. `'p'`, `'jc69'`, or `'poisson'` for amino acids. A model that does not match the data type raises `ValueError`.
- `triangle`: `'full'`, `'lower'`, or `'upper'`.
- `diagonal`: `'zero'`, `'blank'`, or `'max'` (the row maximum).
- `group_diagonal`: Diagonal of the group mean matrix. `'within'` (the within-group mean), `'zero'`, `'blank'`, or `'max'`.
- `long_format`: Also write long-format tables with one row per pair.

To compute the mean distance within and between groups of samples, choose one grouping before running:

- `group_by_delimiter(delimiter, fields)`: Split IDs on `delimiter` and keep the first `fields` parts. For example, `Rattus_rattus_FMNH123` with `'_'` and `2` belongs to `Rattus_rattus`.
- `group_by_regex(pattern)`: Use the first capture group, or the whole match if there is no capture group.
- `group_by_file(path)`: A `.csv` or `.tsv` file with a header row and two columns: sample ID and group.
- `group_auto()`: Detect the groups from the sample IDs.

Run with `from_files(files)` or `from_dir(input_dir)`.

```python
distance = pysegul.AlignmentDistance('nexus', 'dna', 'results/distance', model='k2p')
distance.group_by_delimiter('_', 2)
distance.from_dir('alignments')
```

### AlignmentFiltering

```python
pysegul.AlignmentFiltering(input_fmt, datatype, output_path, output_fmt, is_concat, prefix=None, partition_fmt=None)
```

Set `input_files` or `input_dir`, then call one of:

| Method | Keep alignments that |
| --- | --- |
| `minimal_taxa(percent)` | have at least `percent` (0.0–1.0) of all taxa |
| `minimal_taxa_count(count)` *(upcoming)* | have at least `count` taxa |
| `minimal_length(length)` | are at least `length` sites long |
| `maximal_length(length)` *(upcoming)* | are at most `length` sites long |
| `minimal_parsimony_inf(count)` | have at least `count` parsimony informative sites |
| `maximal_parsimony_inf(count)` *(upcoming)* | have at most `count` parsimony informative sites |
| `percent_informative(percent)` | have at least `percent` (0.0–1.0) parsimony informative sites |
| `missing_data(proportion)` *(upcoming)* | have at most `proportion` (0.0–1.0) missing data (gaps and `?`) |
| `contain_taxa(ids)` | contain all taxa in `ids` |

With `is_concat=True`, the filtered alignments are concatenated. You must then also give a `prefix` and a `partition_fmt`.

### AlignmentSplitting

```python
pysegul.AlignmentSplitting(
    input_path, input_fmt, datatype, output_dir, output_fmt,
    partition_fmt, check_partition, input_partition=None, output_prefix=None,
)
```

Run with `split()`. If `input_partition` is `None`, the partition is read from the input alignment. Set `check_partition=False` to skip partition checking.

### AlignmentSummary

```python
pysegul.AlignmentSummary(input_fmt, datatype, output_path, summary_interval, output_prefix=None, per_locus=False)
```

Run with `from_files(files)` or `from_dir(input_dir)`. `summary_interval` sets the interval for the data matrix completeness summary. Set `per_locus=True` *(upcoming)* to write a summary for each alignment.

### AlignmentTrimming *(upcoming)*

```python
pysegul.AlignmentTrimming(input_fmt, datatype, output_path, output_fmt)
```

Set `input_files` or `input_dir`, then call one of:

- `missing_data(proportion)`: Remove sites whose proportion of missing data is above `proportion` (0.0–1.0).
- `parsimony_inf(count)`: Trim using a threshold of parsimony informative sites.

### AlignmentUnalignment *(upcoming)*

Remove gaps and missing data to get unaligned sequences.

```python
pysegul.AlignmentUnalignment(input_fmt, datatype, output_path, output_fmt='fasta')
```

`output_fmt` can be `'fasta'` or `'fasta-int'`. Run with `from_files(files)` or `from_dir(input_dir)`.

### PartitionConversion

```python
pysegul.PartitionConversion(input_fmt, datatype, output_dir, output_fmt, check_partition)
```

Run with `from_files(files)`. `input_fmt` and `output_fmt` are partition formats. Set `check_partition=False` to skip partition checking.

## Genomics

### ReadSummary

```python
pysegul.ReadSummary(input_fmt, mode, output_path, output_prefix=None)
```

- `input_fmt`: `'auto'`, `'fastq'`, or `'gzip'`.
- `mode`: `'minimal'`, `'default'`, or `'complete'`.

Run with `from_files(files)` or `from_dir(input_dir)`.

### ContigSummary

```python
pysegul.ContigSummary(input_fmt, output_dir, output_prefix=None)
```

`input_fmt` can be `'auto'`, `'fasta'`, or `'gzip'`. Run with `from_files(files)` or `from_dir(input_dir)`.

### MafConversion *(upcoming)*

Convert multiple alignment format (MAF) files to sequence alignments.

```python
pysegul.MafConversion(
    output_path,
    output_fmt='fasta',
    group_by='block',
    bed=None,
    clip=True,
    strand_aware=False,
    full_source_name=False,
    ref_species=None,
    species=None,
    min_block_len=None,
    min_species=None,
    min_score=None,
    write_unmatched=False,
    output_prefix=None,
)
```

- `group_by`: Used without a BED file. `'block'` writes one file per block. `'chrom'` writes one file per reference chromosome.
- `bed`: BED file of loci (`.bed` or `.bed.gz`). Each block goes to every locus it overlaps.
- `clip`: Clip blocks to the loci. Set it to `False` to keep whole blocks. Requires `bed`.
- `strand_aware`: Reverse complement loci on the minus strand. Requires `bed`.
- `write_unmatched`: Write blocks that match no locus to the unmatched directory. Requires `bed`.
- `full_source_name`: Use full MAF source names (for example `hg18.chr7`) as sample names. By default the assembly prefix (`hg18`) is used.
- `ref_species`: Reference species. Defaults to the first species in the first block.
- `species`: List of species to keep. The reference is still used for coordinates.
- `min_block_len`, `min_species`, `min_score`: Skip blocks with fewer reference bases, fewer species (counting the reference), or a lower score.
- `output_prefix`: Name of the locus report CSV written when `bed` is set.

Run with `from_files(files)` or `from_dir(input_dir)`. Errors raise `ValueError`.

### MafSummary *(upcoming)*

```python
pysegul.MafSummary(output_path, output_prefix=None, full_source_name=False, ref_species=None, species=None)
```

Writes per-species and per-file summaries. Run with `from_files(files)` or `from_dir(input_dir)`.

## Sequence

### SequenceAddition *(upcoming)*

Add sequences to existing files. The output is unaligned (gaps and missing data are removed).

```python
pysegul.SequenceAddition(input_fmt, datatype, output_path, output_fmt='fasta', added_only=False)
```

Set `input_files` or `input_dir`, then call one of:

- `add_to_files(dest_files, dest_fmt)`: Add sequences to the destination files that have the same file name (without extension).
- `add_to_dir(dest_dir, dest_fmt)`: Same as above, using all files in `dest_dir`.
- `add_to_file(dest_file, include_filename=False)`: Add all sequences to one file. Set `include_filename=True` to add the input file name to each sequence ID.

`output_fmt` can be `'fasta'` or `'fasta-int'`. Set `added_only=True` to write only the destination files that got new sequences.

### SequenceExtraction

```python
pysegul.SequenceExtraction(input_fmt, datatype, output_path, output_format)
```

Set `input_files` or `input_dir`, then call `extract_regex(regex)` or `extract_id_list(ids)`.

### SequenceFiltering *(upcoming)*

```python
pysegul.SequenceFiltering(input_fmt, datatype, output_path, output_fmt)
```

Set `input_files` or `input_dir`, then call one of:

- `minimal_length(length)`: Remove sequences shorter than `length` (gaps excluded).
- `maximal_length(length)`: Remove sequences longer than `length` (gaps excluded).
- `maximal_gap(proportion)`: Remove sequences whose proportion of gaps is above `proportion` (0.0–1.0).

### IDExtraction

```python
pysegul.IDExtraction(input_fmt, datatype, output_path, map_id, output_prefix=None)
```

Writes the unique sequence IDs. Set `map_id=True` to also write which IDs are in each file. Run with `from_files(files)` or `from_dir(input_dir)`.

### SequenceRemoval

```python
pysegul.SequenceRemoval(input_fmt, datatype, output_path, output_format)
```

Set `input_files` or `input_dir`, then call `remove_regex(regex)` or `remove_id_list(ids)`.

### SequenceRenaming *(upcoming)*

```python
pysegul.SequenceRenaming(input_fmt, datatype, output_path, output_format, dry_run=False)
```

Set `input_files` or `input_dir`, then call one of:

- `replace_id(names)`: Replace IDs using a list of `(original_id, new_id)` tuples.
- `replace_id_from_file(path)`: Replace IDs using a `.csv` or `.tsv` file with a header row and two columns: original ID and new ID.
- `remove_string(text)`: Remove `text` from the IDs.
- `remove_regex(regex, all=False)`: Remove the first match, or every match when `all=True`.
- `replace_string(from_text, to_text)`: Replace `from_text` with `to_text`.
- `replace_regex(regex, to_text, all=False)`: Replace the first match, or every match when `all=True`.

Set `dry_run=True` to log the new IDs without writing any files.

### SequenceTranslation

```python
pysegul.SequenceTranslation(input_fmt, datatype, output_path, output_format, table, reading_frame=None)
```

`table` is an NCBI translation table number as a string (for example `'1'`). If `reading_frame` is `None`, the reading frame is detected for each file, and the output is written to an `RF-<frame>` directory. Run with `from_files(files)` or `from_dir(input_dir)`.

### SequenceTranspose *(upcoming)*

Turn locus alignments into one sequence file per sample. Each output file has all loci for one sample, written to `output_path/<sample_id>/<sample_id>.<ext>`.

```python
pysegul.SequenceTranspose(input_fmt, datatype, output_path, output_fmt='fasta')
```

Use `'fasta'` or `'fasta-int'`, because the transposed sequences are unaligned. Run with `from_files(files)` or `from_dir(input_dir)`.

## Development

Build and test PySEGUL in a virtual environment:

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin pytest
maturin develop
pytest tests
```
