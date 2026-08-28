use uefi::boot;
use uefi::mem::memory_map::{MemoryMapOwned, MemoryType};
use uefi::Status;

pub fn get_memory_map() -> Result<MemoryMapOwned, Status> {
    boot::memory_map(MemoryType::LOADER_DATA)
        .map_err(|error| error.status())
}

pub fn is_usable(memory_type: MemoryType) -> bool {
    matches!(
        memory_type,
        MemoryType::CONVENTIONAL
            | MemoryType::BOOT_SERVICES_CODE
            | MemoryType::BOOT_SERVICES_DATA
    )
}
