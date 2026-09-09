# Contributing

## Setup

Needs a stable Rust toolchain and nothing else — this crate has zero
external dependencies by design (see the comment above `[dependencies]`
in `Cargo.toml`).

```bash
git clone https://github.com/Dev-X25874/topolens
cd topolens
cargo build
cargo test
```

## Before submitting a PR

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI runs the same checks and will fail on any formatting diff, clippy
warning, or test failure.

## Adding a test

New behavior in `src/advisor.rs` or `src/topology.rs` should come with
a test in `tests/basic.rs`, built against a hand-constructed `Topology`
fixture (see `two_socket_topology()` for the pattern). Tests don't
touch real sysfs, so they need to keep passing on any OS.

## Scope

Bug fixes, doc fixes, and small quality-of-life additions (CLI flags,
output formats) are welcome. This is a small, focused tool — please
open an issue before a PR that adds a dependency or changes the
zero-dependency design.
