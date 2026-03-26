use crate::{architecture::Abi, handle::Handle};
use alloc::{vec, vec::Vec};
use ostd::{
    mm::{FallibleVmRead, VmWriter},
    prelude::println,
};
pub fn handle(handle: &Handle, abi: Abi) -> Option<Abi> {
    let mut buffer: Vec<u8> = vec![0; abi.2.try_into().unwrap()];
    let process = handle.thread.upgrade()?.process().upgrade()?;
    let mut reader = process
        .memory
        .reader(abi.1.try_into().ok()?, abi.2.try_into().ok()?)
        .ok()?;
    reader
        .read_fallible(&mut VmWriter::from(buffer.as_mut_slice()))
        .ok()?;
    println!("{}", str::from_utf8(buffer.as_ref()).ok()?);
    Some(Abi(1, 0, 0, 0, 0, 0))
}
