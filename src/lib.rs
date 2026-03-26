#![no_std]
extern crate alloc;
use alloc::sync::Arc;
use spin::Once;
use crate::process::Process;
pub mod architecture;
pub mod kickstart;
pub mod process;
pub mod device;
pub mod request;
pub mod syscall;
pub mod tag;
pub mod exchange;
pub mod handle;
pub mod user;
pub mod event;
static KICKSTART: Once<Arc<Process>> = Once::new();
#[ostd::main]
fn main() {
    KICKSTART.call_once(|| kickstart::new(include_bytes!("../floreum_kickstart")));
}
