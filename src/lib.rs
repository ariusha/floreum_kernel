#![no_std]
#![feature(array_try_map)]
extern crate alloc;
pub mod architecture;
pub mod kickstart;
pub mod process;
pub mod syscall;
pub mod thread;
pub mod user;
pub mod event;
#[ostd::main]
fn kernel_main() {
    let kickstart = kickstart::new(include_bytes!("../floreum_kickstart"));
    kickstart
        .threads
        .read()
        .get(0)
        .unwrap()
        .task()
        .unwrap()
        .run();
}
