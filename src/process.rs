use crate::thread::Thread;
use alloc::{sync::Arc, vec::Vec};
use ostd::{mm::VmSpace, sync::RwMutex};
pub struct Process {
    pub memory: Arc<VmSpace>,
    pub threads: RwMutex<Vec<Arc<Thread>>>,
}
