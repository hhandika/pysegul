# Release Notes

## Unreleased

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

## 0.4.1 (2024-06-27)

- Fix CI issues.

## 0.4.0 (2024-06-27)

- Add sequence features.

## 0.3.0

- Add util module.

## 0.2.1

- Fix concat files does not have extension.

## 0.2.0

- Add all alignment features.
- Add genomic features.

## 0.1.0

- Initial release.
- Add alignment concatenation feature.
