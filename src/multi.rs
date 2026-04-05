use crate::architecture::this::{ArchitectureCluster, ArchitectureHart, ArchitectureNuma};
use spin::Once;
pub struct Cluster {
    pub architecture: ArchitectureCluster,
    pub parent: Option<&'static Cluster>,
    pub children: Once<&'static [Cluster]>,
    pub harts: Once<&'static [Hart]>,
}
pub struct Hart {
    pub architecture: ArchitectureHart,
    pub cluster: &'static Cluster,
    pub numa: &'static Numa,
}
pub struct Route {
    pub target: &'static Numa,
    pub distance: usize,
}
pub struct Numa {
    pub architecture: ArchitectureNuma,
    pub harts: &'static [&'static Hart],
    pub routes: Once<&'static [Route]>,
}
pub static ROOT: Once<&'static Cluster> = Once::new();
pub static HARTS: Once<&'static [Hart]> = Once::new();
pub static NUMAS: Once<&'static [Numa]> = Once::new();
impl Hart {
    pub fn new(
        architecture: ArchitectureHart,
        cluster: &'static Cluster,
        numa: &'static Numa,
    ) -> Self {
        Self {
            architecture,
            cluster,
            numa,
        }
    }
}
