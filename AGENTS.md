# AGENTS.md

Notes for AI coding assistants (Claude Code, Copilot, Cursor, etc.)
working in this repo.

## Build & test

```bash
cargo build --release
cargo test
cargo run --bin topolens -- scan
cargo run --bin bench
```

No network access is required or used at build time — this crate has
zero dependencies (see the comment in `Cargo.toml`). Don't add a
dependency without a strong reason; if one seems necessary, flag it
rather than adding it silently.

## Layout

- `src/lib.rs` — crate root, re-exports.
- `src/topology.rs` — sysfs reading only. No scoring/ranking logic
  belongs here.
- `src/advisor.rs` — all scoring/ranking logic. No file I/O belongs
  here — it should only ever operate on an already-built `Topology`.
- `src/bin/topolens.rs` — CLI, thin wrapper over the library.
- `src/bin/bench.rs` — micro-benchmarks, uses `std::hint::black_box`
  around both inputs and outputs of anything timed, or the compiler
  will optimize the "benchmark" away.
- `tests/basic.rs` — integration tests, run against hand-built
  synthetic `Topology` fixtures. None of this touches real sysfs, so
  tests pass identically on any OS/CI runner.

## Conventions

- Keep `topology.rs` and `advisor.rs` separate: one reads, one scores.
  Don't let scoring logic leak into the sysfs-reading module or vice
  versa.
- Prefer `let-else` + early return over deeply nested `if let`, matching
  the existing style in `topology.rs`/`advisor.rs`.
- Any sysfs read can fail (permissions, hotplug races, non-Linux CI
  runners) — always handle the failure case explicitly rather than
  `.unwrap()`. Every existing sysfs read in this repo returns `Option`
  and degrades gracefully instead of panicking.
- Doc comments (`///`, `//!`) should describe current behavior only.
  If you change what a function does, update its doc comment in the
  same commit — a stale doc comment was an actual bug found and fixed
  in this repo before (see git history on `advisor.rs`).
- No fabricated numbers in benchmark docs. If you haven't run it and
  measured it, don't write a number — say what the benchmark does and
  how to run it instead.

## Before opening a PR

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and
`cargo test` locally — CI (`.github/workflows/ci.yml`) runs the same
three checks and will fail the build on any warning.
