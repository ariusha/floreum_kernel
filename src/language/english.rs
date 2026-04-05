pub const KERNEL_ENTRY_HELLO_WORLD_MESSAGE: &str = "hello world from floreum kernel!";
pub const LIMINE_MEMORY_MAP_RESPONSE_NONE: &str =
    "limine bootloader failed to respond to memory map request!";
pub const LIMINE_MEMORY_MAP_ENTRIES_NONE: &str =
    "limine bootloader failed to provide physical memory!";
pub const LIMINE_MEMORY_HHDM_RESPONSE_NONE: &str =
    "limine bootloader failed to respond to hhdm request!";
pub const LIMINE_MEMORY_MAP_ALLOCATOR_LARGE: &str =
    "limine bootloader failed to provide memory region large enough to store frame allocator!";
pub const ALLOCATOR_CLAIM_MEMORY_ERR: &str =
    "memory allocator failed to claim memory region specified by bootloader!";
pub const ALLOCATOR_OUT_OF_MEMORY_NULL: &str = "memory allocation failed: out of memory!";
pub const ACPI_HANDLER_FAILED_PHYSICAL_MAPPING: &str =
    "acpi handler failed to map physical region!";
pub const ACPI_HANDLER_UNAUTHORISED_ACTION: &str =
    "acpi handler attempted to perform invalid action!";
pub const LIMINE_RSDT_POINTER_RESPONSE_NONE: &str =
    "limine bootloader failed to respond to rsdp request!";
pub const ACPI_HANDLER_FAILED_TABLES_FROM_RSDP: &str =
    "acpi handler failed to parse tables from rsdp!";
pub const ACPI_HANDLER_FAILED_PLATFORM_FROM_TABLE: &str =
    "acpi handler failed to parse platform from tables!";
pub const ACPI_HANDLER_FAILED_PROCESSORS_FROM_PLATFORM: &str =
    "apci handler failed to parse processor info from platform!";
pub const FRAME_ALLOCATOR_DOUBLE_MERGE_ERR: &str =
    "failed to merge siblings while doubling frames!";
pub const FRAME_ALLOCATOR_HIGHER_SPLIT_ERR: &str =
    "failed to split children from higher-order frame!";
pub const FRAME_ALLOCATOR_NOT_INITIALISED: &str =
    "attempted to allocate/deallocate frame before initialising frame allocator!";
pub const FRAME_ALLOCATOR_INVALID_INDEX: &str =
    "attempted to allocate/deallocate specific frame with invalid index!";
pub const FRAME_ALLOCATOR_DEALLOCATE_INVALID_ORDER: &str =
    "attempted to deallocate frame with invalid order!";
pub const KERNEL_EXIT_FAREWELL_MESSAGE: &str = "exiting floreum kernel! good night...";
