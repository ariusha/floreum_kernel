use crate::{architecture::Abi, thread::Thread};
use alloc::sync::Weak;
use core::sync::atomic::Ordering;
pub fn handle(thread: &Weak<Thread>, _abi: Abi) -> Option<Abi> {
    thread.upgrade()?.executing.store(false, Ordering::Relaxed);
    None
}
