use uefi::boot;
use uefi::mem::memory_map::MemoryMapOwned;
use uefi::mem::MemoryType;
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
            | MemoryType::LOADER_CODE
            | MemoryType::LOADER_DATA
    )
}
