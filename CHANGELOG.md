# Release Notes

## 0.5.0 (2026-11-16)

- Depend on the `segul-core` crate instead of `segul`, matching the SEGUL 0.24 library split.
- Add `AlignmentDistance` for pairwise genetic distances, with group mean distances.
- Add `AlignmentTrimming` and `AlignmentUnalignment`.
- Add `MafConversion` and `MafSummary` for multiple alignment format (MAF) files.
- Add `SequenceAddition`, `SequenceFiltering`, and `SequenceTranspose`.
- Finish `SequenceRenaming`, with a dry-run option.
- Add `minimal_taxa_count()`, `maximal_length()`, `maximal_parsimony_inf()`, and `missing_data()` to `AlignmentFiltering`.
- Add the `per_locus` option to `AlignmentSummary`.
- Fix `check_partition` in `AlignmentSplitting` and `PartitionConversion`. It was inverted: `True` skipped partition checking.
- `IDExtraction` with `map_id=True` no longer writes the unique ID file twice.
- Document all features on the SEGUL website (Python API section).
- Build against `segul-core` 0.25 and PyO3 0.30.
- Add `PlotOptions` and a `plot` option to `AlignmentSummary`, `AlignmentDistance`, `ReadSummary`, `ContigSummary`, and `MafSummary`. Charts are written to `<output>/plots` as PNG, SVG, or PDF.
- Add `load_align_report()`, `load_contig_report()`, `load_distance_report()`, and `load_read_report()` to read summary reports as dicts. Read reports include the quality checks for each sample.
- Add `phred` and `dup_sketch_size` to `ReadSummary`.
- Add `extract_id_file()` to `SequenceExtraction`.
- Summary and distance classes now return a dict of output paths. `MafConversion` returns block and locus counts.
- Ship type stubs and docstrings, so editors show parameters and help.
- Add defaults matching the SEGUL CLI: `summary_interval=5`, `sort_sequence=False`, `is_concat=False`, `check_partition=True`, `map_id=False`, `table="1"`, and `dest_fmt="auto"`.

### Breaking Changes

- Require Python 3.9 or later.
- Invalid options raise `ValueError` instead of `PanicException`. Read summary and contig summary errors also raise `ValueError`.
- `AlignmentFiltering` with `is_concat=True` requires `prefix` and `partition_fmt` when created, not when filtering.
- Internal helpers such as `concat_alignments()`, `summarize_reads()`, and `count_min_tax()` are no longer callable from Python. Use `from_files()` or `from_dir()`.
- `MafConversion.convert()` and `MafSummary.summarize()` are no longer callable from Python. Use `from_files()` or `from_dir()`.

## 0.4.1 (2024-06-27)

- Fix CI issues.

## 0.4.0 (2024-06-27)

- Add sequence features.

## 0.3.0 (2024-06-20)

- Add util module.

## 0.2.1 (2024-06-19)

- Fix concat files does not have extension.

## 0.2.0 (2024-06-19)

- Add all alignment features.
- Add genomic features.

## 0.1.0 (2024-06-19)

- Initial release.
- Add alignment concatenation feature.
