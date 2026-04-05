use core::arch::asm;
#[inline]
pub unsafe fn msr_read(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(att_syntax, nostack, preserves_flags)
        )
    };
    ((high as u64) << 32) | (low as u64)
}
#[inline]
pub unsafe fn msr_write(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
            options(att_syntax, nostack, preserves_flags)
        )
    };
}
