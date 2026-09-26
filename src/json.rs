//! Minimal hand-rolled JSON writer for the CLI's `--json` output.
//!
//! This crate is deliberately zero-dependency (see the comment in
//! `Cargo.toml`), so pulling in `serde`/`serde_json` just to print a few
//! objects isn't worth it. This is not a general-purpose JSON library —
//! it only knows how to render the two shapes the CLI needs:
//! `Topology` and a list of `Placement`s.

use crate::advisor::{self, Placement};
use crate::topology::Topology;

/// Escapes a string for embedding inside a JSON string literal.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn quote(s: &str) -> String {
    format!("\"{}\"", escape(s))
}

/// Renders raw topology (NUMA nodes + accelerators) as a JSON object.
///
/// `cpus` is rendered as a collapsed range string (`"0-3,8"`), matching
/// the text-mode output, rather than a raw array — the range form is
/// what a caller is going to want to paste into `taskset`/`numactl`
/// anyway.
pub fn topology_to_json(topo: &Topology) -> String {
    let nodes: Vec<String> = topo
        .nodes
        .values()
        .map(|n| {
            let distance: Vec<String> =
                n.distance.iter().map(|(id, d)| format!("{}:{d}", quote(&id.to_string()))).collect();
            format!(
                "{{\"id\":{},\"cpus\":{},\"distance\":{{{}}}}}",
                n.id,
                quote(&advisor::cpulist_to_range_str(&n.cpus)),
                distance.join(",")
            )
        })
        .collect();

    let accels: Vec<String> = topo
        .accelerators
        .iter()
        .map(|a| {
            format!(
                "{{\"bdf\":{},\"class\":{},\"vendor\":{},\"device\":{},\"numa_node\":{}}}",
                quote(&a.bdf),
                quote(&a.class),
                quote(&a.vendor),
                quote(&a.device),
                a.numa_node.map(|n| n.to_string()).unwrap_or_else(|| "null".into())
            )
        })
        .collect();

    format!("{{\"numa_nodes\":[{}],\"accelerators\":[{}]}}", nodes.join(","), accels.join(","))
}

/// Renders a set of placement recommendations as a JSON array.
pub fn placements_to_json(placements: &[Placement]) -> String {
    let items: Vec<String> = placements
        .iter()
        .map(|p| {
            let ranked: Vec<String> = p
                .ranked_nodes
                .iter()
                .map(|(id, d)| format!("{{\"node\":{id},\"distance\":{d}}}"))
                .collect();
            format!(
                "{{\"accelerator\":{},\"verdict\":{},\"pin_cpus\":{},\"ranked_nodes\":[{}]}}",
                quote(&p.accelerator.bdf),
                quote(p.verdict()),
                quote(&advisor::cpulist_to_range_str(&p.recommended_cpus)),
                ranked.join(",")
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}
