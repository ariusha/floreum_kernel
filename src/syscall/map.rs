use crate::{architecture::Abi, handle::Handle, process::Thread};
use alloc::sync::Weak;
use ostd::{mm::{CachePolicy, FrameAllocOptions, PAGE_SIZE, PageFlags, PageProperty}, sync::Waiter, task::disable_preempt};
pub fn handle(handle: &Handle, abi: Abi) -> Option<Abi> {
    let offset: usize = abi.1.try_into().ok()?;
    let length: usize = abi.2.try_into().ok()?;
    let guard = disable_preempt();
    let thread_arc = handle.thread
        .upgrade()?;
    let process_arc = thread_arc.process().upgrade()?;
    let mut cursor = 
        process_arc
        .memory
        .cursor_mut(
            &guard,
            &((offset * PAGE_SIZE)..(offset + length)),
        )
        .ok()?;
    let properties = PageProperty::new_user(PageFlags::RWX, if abi.3 != 0 {
        CachePolicy::Writeback
    } else {
        CachePolicy::Writethrough
    });
    for frame in FrameAllocOptions::new().alloc_segment(length).ok()? { // todo
        cursor.map(frame.into(), properties);
    }
    Some(Abi(1, 0, 0, 0, 0, 0))
}
