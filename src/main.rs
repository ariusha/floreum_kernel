#![no_std]
#![no_main]
#![feature(allocator_api)]
extern crate alloc;
use crate::{
    architecture::this::alloc_initialise,
    language::this::{
        KERNEL_ENTRY_HELLO_WORLD_MESSAGE, KERNEL_EXIT_FAREWELL_MESSAGE,
        LIMINE_MEMORY_MAP_RESPONSE_NONE,
    },
    limine::MEMORY_MAP,
    memory::GLOBAL_ALLOCATOR,
};
use ::limine::BaseRevision;
use core::alloc::{GlobalAlloc, Layout};
use spinning_top::RawSpinlock;
use talc::{ErrOnOom, Talc, Talck};
mod aligned;
mod architecture;
mod frame;
mod language;
mod limine;
mod memory;
mod multi;
mod panic;
mod physical;
mod print;
mod process;
mod scheduler;
mod segment;
mod tagmap;
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::with_revision(BaseRevision::MAX_SUPPORTED);
pub unsafe fn main() {
    let memmap = MEMORY_MAP
        .response()
        .expect(LIMINE_MEMORY_MAP_RESPONSE_NONE);
    unsafe {
        memory::initialise(memmap.entries());
    }
    println!("{}", KERNEL_ENTRY_HELLO_WORLD_MESSAGE);
    let layout = Layout::from_size_align(8, 8).unwrap();
    assert!(!unsafe { GLOBAL_ALLOCATOR.alloc(layout) }.is_null());
    println!("test");
    unsafe {
        alloc_initialise();
    };
    println!("{}", KERNEL_EXIT_FAREWELL_MESSAGE);
}

use crate::architecture::this::{interrupts_are_enabled, interrupts_disable, interrupts_enable};
#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved = interrupts_are_enabled();
    if saved {
        unsafe { interrupts_disable() };
    }
    let ret = f();
    if saved {
        unsafe { interrupts_enable() };
    }
    ret
}
