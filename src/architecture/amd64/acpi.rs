use crate::{
    architecture::this::{ArchitectureCluster, ArchitectureHart, ArchitectureNuma},
    language::this::{
        ACPI_HANDLER_FAILED_PHYSICAL_MAPPING, ACPI_HANDLER_FAILED_PLATFORM_FROM_TABLE,
        ACPI_HANDLER_FAILED_PROCESSORS_FROM_PLATFORM, ACPI_HANDLER_FAILED_TABLES_FROM_RSDP,
        ACPI_HANDLER_UNAUTHORISED_ACTION, LIMINE_RSDT_POINTER_RESPONSE_NONE,
    },
    limine::RSDP,
    multi::{Cluster, Hart, Numa},
    physical::Physical,
    println,
};
use acpi::{
    AcpiTables, Handler, PciAddress, PhysicalMapping,
    aml::AmlError,
    platform::{AcpiPlatform, ProcessorState},
};
use alloc::{boxed::Box, vec::Vec};
use core::ptr::NonNull;
use spin::Once;
#[derive(Clone)]
pub struct StubAcpiHandler;
impl Handler for StubAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new(
                <u64 as Into<Physical>>::into(physical_address as u64).to_virtual() as *mut T,
            )
            .expect(ACPI_HANDLER_FAILED_PHYSICAL_MAPPING),
            region_length: size,
            mapped_length: size,
            handler: StubAcpiHandler,
        }
    }
    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
    fn read_u8(&self, _address: usize) -> u8 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_u16(&self, _address: usize) -> u16 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_u32(&self, _address: usize) -> u32 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_u64(&self, _address: usize) -> u64 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_u8(&self, _address: usize, _value: u8) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_u16(&self, _address: usize, _value: u16) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_u32(&self, _address: usize, _value: u32) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_u64(&self, _address: usize, _value: u64) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_io_u8(&self, _port: u16) -> u8 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_io_u16(&self, _port: u16) -> u16 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_io_u32(&self, _port: u16) -> u32 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_io_u8(&self, _port: u16, _value: u8) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_io_u16(&self, _port: u16, _value: u16) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_io_u32(&self, _port: u16, _value: u32) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_pci_u8(&self, _address: PciAddress, _offset: u16) -> u8 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_pci_u16(&self, _address: PciAddress, _offset: u16) -> u16 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn read_pci_u32(&self, _address: PciAddress, _offset: u16) -> u32 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_pci_u8(&self, _address: PciAddress, _offset: u16, _value: u8) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_pci_u16(&self, _address: PciAddress, _offset: u16, _value: u16) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn write_pci_u32(&self, _address: PciAddress, _offset: u16, _value: u32) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn nanos_since_boot(&self) -> u64 {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn stall(&self, _microseconds: u64) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
    fn sleep(&self, _milliseconds: u64) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }

    fn create_mutex(&self) -> acpi::Handle {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }

    fn acquire(&self, _mutex: acpi::Handle, _timeout: u16) -> Result<(), AmlError> {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }

    fn release(&self, _mutex: acpi::Handle) {
        panic!("{}", ACPI_HANDLER_UNAUTHORISED_ACTION)
    }
}
pub fn heirarchy() -> (&'static Cluster, &'static [Hart]) {
    let platform = unsafe {
        AcpiPlatform::new(
            AcpiTables::from_rsdp(
                StubAcpiHandler,
                RSDP.response()
                    .expect(LIMINE_RSDT_POINTER_RESPONSE_NONE)
                    .address as usize,
            )
            .expect(ACPI_HANDLER_FAILED_TABLES_FROM_RSDP),
            StubAcpiHandler,
        )
    }
    .expect(ACPI_HANDLER_FAILED_PLATFORM_FROM_TABLE);
    let processor_info = platform
        .processor_info
        .expect(ACPI_HANDLER_FAILED_PROCESSORS_FROM_PLATFORM);
    let numa = Box::leak(Box::new(Numa {
        architecture: ArchitectureNuma::new(),
        harts: &[],
        routes: Once::new(),
    }));
    let root = Box::leak(Box::new(Cluster {
        architecture: ArchitectureCluster::new(),
        parent: None,
        children: Once::new(),
        harts: Once::new(),
    }));
    let harts: &'static mut [Hart] = processor_info
        .application_processors
        .iter()
        .chain([processor_info.boot_processor].iter())
        .map(|processor| {
            Hart::new(
                ArchitectureHart::new(
                    !processor.is_ap,
                    matches!(processor.state, ProcessorState::Disabled),
                    processor.processor_uid,
                    processor.processor_uid,
                ),
                root,
                numa,
            )
        })
        .collect::<Vec<Hart>>()
        .leak();
    println!("WARNING: USING SIMPLIFIED PROCESSOR HEIRARCHY!");
    (root, harts)
}
