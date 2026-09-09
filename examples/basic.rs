//! Scans the local machine and prints a placement recommendation for
//! every accelerator found, plus a ready-to-paste `numactl` command.
//!
//! Run with:
//!   cargo run --example basic

use topolens::{advisor, Topology};

fn main() {
    let topo = Topology::scan();

    if topo.accelerators.is_empty() {
        println!("no accelerators found on this host");
        return;
    }

    for placement in advisor::recommend_all(&topo) {
        let cpus = advisor::cpulist_to_range_str(&placement.recommended_cpus);
        let home_node = placement
            .ranked_nodes
            .first()
            .map(|(id, _)| id.to_string())
            .unwrap_or_else(|| "0".into());

        println!("{}", placement.accelerator.bdf);
        println!("  verdict: {}", placement.verdict());
        println!("  pin cpus: {cpus}");
        println!("  numactl --physcpubind={cpus} --membind={home_node}");
        println!();
    }
}
