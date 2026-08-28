use uefi::boot;
use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned, MemoryType};
use uefi::Status;

pub fn get_memory_map() -> Result<MemoryMapOwned, Status> {
    boot::memory_map(MemoryType::LOADER_DATA)
        .map_err(|error| error.status())
}

pub fn usable_regions(
    memory_map: &MemoryMapOwned,
) -> impl Iterator<Item = (u64, u64)> + '_ {
    memory_map
        .entries()
        .filter(|entry| entry.ty == MemoryType::CONVENTIONAL)
        .map(|entry| {
            let start = entry.phys_start;
            let size = entry.page_count * 4096;
            let end = start + size;

            (start, end)
        })
}
