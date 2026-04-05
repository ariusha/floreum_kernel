mod acpi;
mod flow;
mod idt;
mod interrupts;
mod local;
mod memory;
mod msr;
mod multi;
mod port;
mod print;
mod process;
use crate::{
    architecture::amd64::acpi::heirarchy,
    main,
    multi::{HARTS, ROOT},
};
pub use flow::*;
pub use interrupts::*;
pub use local::*;
pub use memory::*;
pub use msr::*;
pub use multi::*;
pub use port::*;
pub use print::*;
pub use process::*;
#[unsafe(export_name = "_start")]
pub unsafe extern "C" fn entry() -> ! {
    unsafe {
        print::initialise();
        main();
    }
    flow_exit()
}
pub unsafe fn alloc_initialise() {
    let (root, harts) = heirarchy();
    ROOT.call_once(|| root);
    HARTS.call_once(|| harts);
}
