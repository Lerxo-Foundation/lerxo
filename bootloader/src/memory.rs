use uefi::boot;
use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned, MemoryType};
use uefi::Status;

pub const PAGE_SIZE: u64 = 4096;

/// Retrieves the current UEFI memory map.
pub fn get_memory_map() -> Result<MemoryMapOwned, Status> {
    boot::memory_map(MemoryType::LOADER_DATA)
        .map_err(|error| error.status())
}

/// Returns the physical ranges currently marked as conventional memory.
pub fn usable_regions(
    memory_map: &MemoryMapOwned,
) -> impl Iterator<Item = (u64, u64)> + '_ {
    memory_map
        .entries()
        .filter(|entry| entry.ty == MemoryType::CONVENTIONAL)
        .filter_map(|entry| {
            let start = entry.phys_start;
            let size = entry.page_count.checked_mul(PAGE_SIZE)?;
            let end = start.checked_add(size)?;

            Some((start, end))
        })
}
