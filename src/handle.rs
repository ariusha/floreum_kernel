use alloc::sync::Weak;
use ostd::sync::Waiter;
use crate::process::Thread;
pub struct Handle<'a, 'b> {
    pub thread: &'a Weak<Thread>,
    waiter: &'b Waiter,
}
impl<'a, 'b> Handle<'a, 'b> {
    pub fn new(thread: &'a Weak<Thread>, waiter: &'b Waiter) -> Self {
        Self {thread, waiter}
    }
    pub fn park(&self) -> Option<()> {
        unsafe {self.thread.upgrade()?.mark_parked(true)};
        self.waiter.wait();
        Some(())
    }
}