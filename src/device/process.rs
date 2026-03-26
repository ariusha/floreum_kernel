use core::sync::atomic::AtomicUsize;
use alloc::{sync::Arc, vec::Vec};
use crate::device::{Device, KernelDevice};
static COUNTER: AtomicUsize = AtomicUsize::new(0);
static HANDLER: fn(Vec<u8>) -> Vec<u8> = |request| {
    COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Vec::new()
};
pub fn new() -> Arc<Device> {
    KernelDevice::new(&HANDLER)
}