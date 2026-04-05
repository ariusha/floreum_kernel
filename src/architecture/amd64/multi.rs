use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

pub struct ArchitectureCluster {}
pub struct ArchitectureHart {
    pub bootstrap: bool,
    pub disabled: bool,
    pub lapic_id: u32,
    pub processor_id: u32,
    pub gdt: GlobalDescriptorTable,
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
}
pub struct ArchitectureNuma {}
impl ArchitectureCluster {
    pub fn new() -> Self {
        Self {}
    }
}
impl ArchitectureHart {
    pub fn new(bootstrap: bool, disabled: bool, lapic_id: u32, processor_id: u32) -> Self {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code = gdt.append(Descriptor::kernel_code_segment());
        let kernel_data = gdt.append(Descriptor::kernel_data_segment());
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());
        Self {
            bootstrap,
            disabled,
            lapic_id,
            processor_id,
            gdt,
            kernel_code,
            kernel_data,
            user_code,
            user_data,
        }
    }
}
impl ArchitectureNuma {
    pub fn new() -> Self {
        Self {}
    }
}
