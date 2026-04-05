use core::arch::asm;
#[inline]
pub unsafe fn interrupts_enable() {
    unsafe { asm!("sti", options(preserves_flags, nostack)) };
}
#[inline]
pub unsafe fn interrupts_disable() {
    unsafe { asm!("cli", options(preserves_flags, nostack)) };
}
#[inline]
pub fn interrupts_are_enabled() -> bool {
    let rflags: u64;
    unsafe { asm!("pushf; pop {}", out(reg) rflags, options(nomem, preserves_flags)) };
    (rflags & (1 << 9)) != 0
}
