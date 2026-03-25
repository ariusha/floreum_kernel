use crate::{event::Event, process::Process, user::user_task};
use alloc::sync::{Arc, Weak};
use core::sync::atomic::{AtomicBool, Ordering};
use crossbeam::queue::SegQueue;
use ostd::{
    arch::cpu::context::{CpuException, UserContext},
    sync::RwLock,
    task::{Task, TaskOptions},
};
pub struct Thread {
    pub process: Weak<Process>,
    pub executing: AtomicBool,
    pub context: RwLock<UserContext>,
    pub events: SegQueue<Event>,
    pub exception: RwLock<Option<CpuException>>,
}
impl Thread {
    pub fn task(self: &Arc<Self>) -> Option<Arc<Task>> {
        let weak = Arc::downgrade(self);
        Some(Arc::new(
            TaskOptions::new(|| unsafe {
                user_task(weak);
            })
            .build()
            .ok()?,
        ))
    }
    pub unsafe fn self_abort(&self) -> Option<()> {
        self.executing.store(false, Ordering::Relaxed);
        None
    }
}
