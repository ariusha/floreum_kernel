use crate::architecture::this::{msr_read, msr_write};
use core::arch::asm;
pub const IA32_GS_BASE: u32 = 0xC000_0101;
pub const IA32_KERNEL_GS_BASE: u32 = 0xC000_0102;
pub fn local_read() -> u64 {
    unsafe { msr_read(IA32_KERNEL_GS_BASE) }
}
pub unsafe fn local_write(value: u64) {
    unsafe { msr_write(IA32_KERNEL_GS_BASE, value) };
}
pub unsafe fn local_swapgs() {
    unsafe { asm!("swapgs", options(att_syntax, nostack, preserves_flags)) };
}
