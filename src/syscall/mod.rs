pub mod abort;
pub mod debug;
pub mod map;
use crate::{architecture::Abi, thread::Thread};
use alloc::sync::Weak;
const TAG_ABORT: u64 = 0;
const TAG_DEBUG: u64 = 1024;
pub fn handle(thread: &Weak<Thread>, abi: Abi) -> Option<Abi> {
    match abi.0 {
        TAG_ABORT => abort::handle(thread, abi),
        TAG_DEBUG => debug::handle(thread, abi),
        _ => None,
    }
}
