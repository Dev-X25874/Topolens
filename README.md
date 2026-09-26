# topolens

A small Rust tool that reads a Linux machine's NUMA/PCIe topology from
sysfs and recommends which CPU cores to pin near a given GPU or
accelerator device.

## What it does

- Reads NUMA node and CPU info from `/sys/devices/system/node/*`,
  including the kernel's SLIT distance table (relative memory-access
  cost between nodes).
- Finds GPUs and other accelerator-class PCIe devices from
  `/sys/bus/pci/devices/*`, along with each device's reported NUMA
  affinity.
- Scores every NUMA node's distance to each accelerator's home node
  and outputs a ranked recommendation, a plain-English verdict, and a
  ready-to-paste `numactl` command.

No `hwloc`, no netlink, zero external crates — just the sysfs files
the kernel already exposes.

## Why

Placing a workload's CPU threads on the wrong NUMA node relative to
its GPU can cost real throughput on memory-bandwidth-bound work. This
tool automates the "which cores are actually close to this GPU"
question instead of doing it by hand with `lstopo`/`numactl -H`.

## Install

```console
$ cargo install --path .
```

Or build without installing:

```console
$ cargo build --release
$ ./target/release/topolens scan
```

## Usage

```console
$ topolens scan
NUMA nodes: 2
  node0  cpus=0-15
  node1  cpus=16-31
Accelerators: 1
  0000:81:00.0  class=0x030200  vendor=0x10de  device=0x2331  numa_node=1

$ topolens recommend
0000:81:00.0
  verdict: local: pin here, no cross-node hop
  pin cpus: 16-31
  numactl:  numactl --physcpubind=16-31 --membind=1
```

Add `--json` to either command for machine-readable output, e.g. for
feeding a scheduler instead of a human:

```console
$ topolens recommend --json
[{"accelerator":"0000:81:00.0","verdict":"local: pin here, no cross-node hop","pin_cpus":"16-31","ranked_nodes":[{"node":1,"distance":10},{"node":0,"distance":21}]}]
```

As a library:

```rust
use topolens::{Topology, advisor};

let topo = Topology::scan();
for placement in advisor::recommend_all(&topo) {
    println!("{}: {}", placement.accelerator.bdf, placement.verdict());
}
```

## Scope vs other tools

hwloc/`lstopo` is a general-purpose topology library that builds a full
hardware graph on every run. This tool only reads the handful of sysfs
paths it needs for CPU-to-accelerator scoring — narrower in scope, not
a replacement for hwloc.

## What this isn't

It doesn't schedule anything and doesn't talk to any external control
plane — it's just a topology-to-recommendation calculator. Wiring the
output into an actual scheduler is left to the caller.

## Performance

See [BENCHMARKS.md](./BENCHMARKS.md) for micro-benchmark numbers and
how to reproduce them.

## Tests

```console
$ cargo test
```

Tests run against synthetic topology fixtures (no real sysfs
required), so they pass regardless of the machine running them.
