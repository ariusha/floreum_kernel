use crate::{architecture::Abi, handle::Handle};
use alloc::sync::Weak;
pub fn handle(handle: &Handle, _abi: Abi) -> Option<Abi> {
    handle.park()?;
    Some(Abi(1, 0, 0, 0, 0, 0))
}
