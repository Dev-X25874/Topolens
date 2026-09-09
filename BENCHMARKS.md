# Benchmarks

No numbers are published here — none have actually been measured and
verified on real hardware yet. Below is what each benchmark measures
and how to run it yourself.

## Advisory logic latency

`src/bin/bench.rs` measures the cost of calling this crate in-process,
against synthetic topology fixtures (zero I/O, same fixtures as
`tests/basic.rs`).

```bash
cargo build --release
./target/release/bench
```

Prints median/min/p99/max nanoseconds-per-call for `recommend_all`,
`recommend`, `verdict`, and `cpulist_to_range_str` across a few fixture
shapes. Numbers will vary by machine — run it on your own hardware to
get real figures.

## Cold-start latency

`benchmarks/cold_start.sh` measures full process lifetime (binary
load, sysfs reads, output, exit) for `topolens`, and optionally
compares it against `lstopo` if hwloc is installed.

```bash
sudo apt install hwloc hyperfine   # optional, for the lstopo comparison
cargo build --release
bash benchmarks/cold_start.sh
```

Results are saved to `benchmarks/cold_start_results.md` and `.json`
when `hyperfine` is available.

## NUMA placement throughput

`benchmarks/throughput_pinned_vs_unpinned.py` calls `topolens
recommend` to get a `numactl` invocation, then runs a memory-bandwidth
workload (large numpy array ops) unpinned, pinned per the
recommendation, and pinned to the wrong node — printing GB/s for each.

```bash
pip install numpy
sudo apt install numactl
cargo build --release
python3 benchmarks/throughput_pinned_vs_unpinned.py
```

On a single-socket machine, all three conditions should come out
roughly equal — that's the expected/correct result, not a failure,
since there's only one NUMA node to pin to.
