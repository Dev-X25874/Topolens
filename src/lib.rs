//! `topolens`: turns a machine's raw NUMA/PCIe topology into
//! placement decisions for AI/ML workloads — which cores to pin near
//! which accelerator, and how much it'll cost you if you don't.
//!
//! Reads topology straight from sysfs on the local Linux host, so the
//! data reflects the real hardware rather than a guess made a few
//! abstraction layers up.
//!
//! ```no_run
//! use topolens::{Topology, advisor};
//!
//! let topo = Topology::scan();
//! for placement in advisor::recommend_all(&topo) {
//!     println!(
//!         "{}: {} -> cpus {}",
//!         placement.accelerator.bdf,
//!         placement.verdict(),
//!         advisor::cpulist_to_range_str(&placement.recommended_cpus),
//!     );
//! }
//! ```

pub mod advisor;
pub mod topology;

pub use advisor::Placement;
pub use topology::{AccelDevice, NumaNode, Topology};
