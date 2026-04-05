use crate::{architecture::this::flow_exit, println};
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
    flow_exit()
}
