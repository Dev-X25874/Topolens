use std::collections::BTreeMap;
use topolens::advisor::recommend;
use topolens::json::{placements_to_json, topology_to_json};
use topolens::topology::{AccelDevice, NumaNode, Topology};

fn sample_topology() -> Topology {
    let mut nodes = BTreeMap::new();

    let mut d0 = BTreeMap::new();
    d0.insert(0, 10);
    d0.insert(1, 21);
    nodes.insert(0, NumaNode { id: 0, cpus: vec![0, 1, 2, 3], distance: d0 });

    let accel = AccelDevice {
        bdf: "0000:81:00.0".into(),
        class: "0x030200".into(),
        numa_node: Some(0),
        vendor: "0x10de".into(),
        device: "0x2331".into(),
    };

    Topology { nodes, accelerators: vec![accel] }
}

#[test]
fn topology_json_has_expected_shape() {
    let topo = sample_topology();
    let out = topology_to_json(&topo);

    assert!(out
        .contains("\"numa_nodes\":[{\"id\":0,\"cpus\":\"0-3\",\"distance\":{\"0\":10,\"1\":21}}]"));
    assert!(out.contains("\"bdf\":\"0000:81:00.0\""));
    assert!(out.contains("\"numa_node\":0"));
}

#[test]
fn topology_json_renders_null_for_unknown_numa_node() {
    let mut topo = sample_topology();
    topo.accelerators[0].numa_node = None;

    let out = topology_to_json(&topo);
    assert!(out.contains("\"numa_node\":null"));
}

#[test]
fn placements_json_has_expected_shape() {
    // Needs a second NUMA node so `ranked_nodes` has more than one entry
    // to check ordering against.
    let mut topo = sample_topology();
    let mut d1 = BTreeMap::new();
    d1.insert(0, 21);
    d1.insert(1, 10);
    topo.nodes.insert(1, NumaNode { id: 1, cpus: vec![4, 5, 6, 7], distance: d1 });

    let placement = recommend(&topo, &topo.accelerators[0]);
    let out = placements_to_json(&[placement]);

    assert!(out.starts_with('['));
    assert!(out.ends_with(']'));
    assert!(out.contains("\"accelerator\":\"0000:81:00.0\""));
    assert!(out.contains("\"verdict\":\"local: pin here, no cross-node hop\""));
    assert!(out.contains("\"pin_cpus\":\"0-3\""));
    assert!(out
        .contains("\"ranked_nodes\":[{\"node\":0,\"distance\":10},{\"node\":1,\"distance\":21}]"));
}

#[test]
fn empty_placements_is_an_empty_json_array() {
    assert_eq!(placements_to_json(&[]), "[]");
}

#[test]
fn special_characters_are_escaped() {
    let mut topo = sample_topology();
    // Synthetic value to exercise the string-escaping path — real sysfs
    // data never contains quotes/backslashes, but the writer still has
    // to not produce broken JSON if it ever does.
    topo.accelerators[0].device = "weird\"device\\name\nwith a newline".into();

    let out = topology_to_json(&topo);
    assert!(out.contains("\"device\":\"weird\\\"device\\\\name\\nwith a newline\""));
}
