# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added
- `--json` flag on `topolens scan` and `topolens recommend` for
  machine-readable output, via a small hand-rolled JSON writer
  (`src/json.rs`) — no new dependency added.

### Fixed
- `bench.rs`: the "far verdict" benchmark now actually exercises the
  far-placement code path (previously it measured a placement that
  was always local, so the label didn't match what was measured).
- `advisor.rs`: doc comment for the no-NUMA-affinity case corrected —
  it returns an empty result, not a placement on node 0.

### Changed
- Removed unverified/fabricated numbers from `BENCHMARKS.md` — it now
  only documents what each benchmark measures and how to run it.
- Rewrote `README.md` as a plain project description.
- Added `AGENTS.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, a GitHub
  Actions CI workflow, an `examples/basic.rs`, and `rustfmt.toml`.

## [0.1.0]

Initial version: sysfs-based NUMA/PCIe topology scanning, distance-based
placement scoring, `topolens` CLI, and a micro-benchmark suite.
