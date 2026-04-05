use core::arch::asm;
#[inline]
pub unsafe fn flow_wait() {
    unsafe { asm!("sti; hlt", options(nomem, nostack)) };
}
#[inline]
pub fn flow_exit() -> ! {
    unsafe { asm!("sti; int 255", options(nomem, nostack, preserves_flags)) };
    loop {}
}
