use crate::{
    architecture::this::{AddressSpace, ArchitectureProcess, ArchitectureThread, ThreadRegisters},
    multi::Hart,
    scheduler::ThreadSchedulingData,
    tagmap::TagMap,
};
use alloc::{collections::vec_deque::VecDeque, rc::Weak, sync::Arc};
use core::sync::atomic::AtomicUsize;
use spin::Mutex;
pub struct Thread {
    pub architecture: ArchitectureThread,
    pub process: Weak<Process>,
    pub scheduling: ThreadSchedulingData,
    pub registers: Mutex<Result<ThreadRegisters, &'static Hart>>,
}
pub enum Queues {
    Sending(VecDeque<Weak<Thread>>),
    Receiving(VecDeque<Weak<Thread>>),
}
pub struct Process {
    pub architecture: ArchitectureProcess,
    pub threads: Mutex<TagMap<Arc<Thread>>>,
    pub space: AddressSpace,
    pub ports: Mutex<TagMap<Arc<Mutex<Queues>>>>,
}
