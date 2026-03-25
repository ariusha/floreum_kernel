#[derive(Clone, Copy)]
pub struct Abi(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);
pub mod x86_64;
pub mod this {
    #[cfg(target_arch = "x86_64")]
    pub use crate::architecture::x86_64::*;
}
