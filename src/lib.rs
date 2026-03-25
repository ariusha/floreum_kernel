#![no_std]
extern crate alloc;
use alloc::sync::Arc;
use spin::Once;
use crate::process::Process;
pub mod architecture;
pub mod kickstart;
pub mod process;
pub mod syscall;
pub mod handle;
pub mod user;
pub mod event;
static KICKSTART: Once<Arc<Process>> = Once::new();
#[ostd::main]
fn kernel_main() {
    let kickstart = kickstart::new(include_bytes!("../floreum_kickstart"));
}
