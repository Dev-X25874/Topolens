#!/usr/bin/env bash
# cold_start.sh — compares topolens cold-start latency against lstopo (hwloc).
#
# What this measures:
#   The full process lifetime: binary load, sysfs reads, output, exit.
#   This is what a scheduler pays every time it calls the tool as a subprocess.
#
# Prerequisites:
#   cargo build --release              (builds topolens)
#   sudo apt install hwloc hyperfine   (or: cargo install hyperfine)
#
# Run:
#   bash benchmarks/cold_start.sh
#
# On a machine without a GPU you'll still get correct timing numbers —
# both tools simply report "no accelerators / no devices found."

set -euo pipefail

BINARY="./target/release/topolens"

if [[ ! -x "$BINARY" ]]; then
    echo "Build first: cargo build --release"
    exit 1
fi

echo ""
echo "=== Cold-start latency: topolens vs lstopo (hwloc) ==="
echo ""

if command -v hyperfine &>/dev/null; then
    hyperfine \
        --warmup 10 \
        --runs 200 \
        --export-markdown benchmarks/cold_start_results.md \
        --export-json    benchmarks/cold_start_results.json \
        "$BINARY recommend" \
        "$BINARY scan" \
        "lstopo --no-graphics --of txt" \
        "lstopo-no-graphics --of txt"

    echo ""
    echo "Results saved to benchmarks/cold_start_results.md"
    echo "and benchmarks/cold_start_results.json"

else
    # Fallback: plain bash timing loop when hyperfine isn't installed.
    echo "hyperfine not found — using bash timing (less accurate, no stats)"
    echo ""

    bench_plain() {
        local label="$1"; shift
        local cmd=("$@")
        local total=0
        local runs=50

        for _ in $(seq 1 $runs); do
            local start
            start=$(date +%s%N)
            "${cmd[@]}" &>/dev/null
            local end
            end=$(date +%s%N)
            total=$(( total + end - start ))
        done

        local avg=$(( total / runs / 1000000 ))
        printf "%-50s  avg %d ms over %d runs\n" "$label" "$avg" "$runs"
    }

    bench_plain "topolens recommend" "$BINARY" recommend
    bench_plain "topolens scan"      "$BINARY" scan

    if command -v lstopo-no-graphics &>/dev/null; then
        bench_plain "lstopo-no-graphics" lstopo-no-graphics --of txt
    else
        echo "(lstopo not installed — install with: sudo apt install hwloc)"
    fi
fi

echo ""
echo "Why this matters:"
echo "  A placement oracle that runs fast enough is safe to call per scheduling"
echo "  decision. topolens reads only what it needs: node/*/cpulist,"
echo "  node/*/distance, and pci/devices/*/class — roughly a dozen file opens"
echo "  total. Compare the numbers above to see the actual difference on this"
echo "  machine."
echo ""
