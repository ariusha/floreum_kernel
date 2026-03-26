pub mod abort;
pub mod debug;
pub mod map;
use crate::{architecture::Abi, handle::Handle};
const TAG_ABORT: u64 = 0;
const TAG_MAP: u64 = 1;
const TAG_DEBUG: u64 = 1024;
pub fn handle(handle: &Handle, abi: Abi) -> Option<Abi> {
    match abi.0 {
        TAG_ABORT => abort::handle(handle, abi),
        TAG_DEBUG => debug::handle(handle, abi),
        TAG_MAP => map::handle(handle, abi),
        _ => None,
    }
}
