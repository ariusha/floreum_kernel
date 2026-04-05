pub mod amd64;
pub mod this {
    #[cfg(target_arch = "x86_64")]
    pub use crate::architecture::amd64::*;
}
