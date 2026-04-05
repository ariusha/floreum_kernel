use crate::architecture::this::{port_in8, port_out8};
use core::fmt::{self, Write};
use spin::Mutex;
const PORT_COM1: u16 = 0x3f8;
const PORT_POST: u16 = 0x80;
pub unsafe fn initialise() {
    for port_write in [
        (PORT_COM1 + 1, 0x00), // disable irqs
        (PORT_COM1 + 3, 0x80), // access divisor latch
        (PORT_COM1 + 0, 0x01), // set divisor low to 1 115200 baud
        (PORT_COM1 + 1, 0x00), // set divisor high to 0
        (PORT_COM1 + 3, 0x03), // 8 bits, no parity, one stop bit
        (PORT_COM1 + 2, 0xC7), // enable & clear fifo with 14-byte threshold
        (PORT_COM1 + 4, 0x0B), // enable irqs
    ] {
        unsafe { port_out8(port_write.0, port_write.1) };
    }
}
pub struct Writer;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                while port_in8(PORT_COM1 + 5) & 0x20 == 0 {
                    port_out8(PORT_POST, 0);
                }
                port_out8(PORT_COM1, byte)
            };
        }
        Ok(())
    }
}
pub static WRITER: Mutex<Writer> = Mutex::new(Writer);
