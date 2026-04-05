use x86_64::structures::idt::InterruptDescriptorTable;
pub static IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
pub unsafe fn initialise() {
    todo!()
}
