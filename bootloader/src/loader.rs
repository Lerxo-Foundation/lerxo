use core::ptr::NonNull;

use uefi::boot::{self, AllocateType, MemoryType, PAGE_SIZE};
use uefi::Status;

use crate::elf::KernelElf;

/// A loaded ELF segment.
pub struct LoadedSegment {
    /// Virtual address specified by the ELF program header.
    pub virtual_address: u64,

    /// Address where the segment was loaded in physical memory.
    pub physical_address: NonNull<u8>,

    /// Size of the segment in memory.
    pub memory_size: usize,
}

/// Loads all PT_LOAD segments of the kernel into memory.
pub fn load_segments(
    kernel: &KernelElf<'_>,
) -> Result<alloc::vec::Vec<LoadedSegment>, Status> {
    let mut segments = alloc::vec::Vec::new();

    for segment in kernel.loadable_segments() {
        let memory_size = segment.p_memsz as usize;

        if memory_size == 0 {
            continue;
        }

        let pages = memory_size
            .div_ceil(PAGE_SIZE);

        let address = boot::allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            pages,
        )?;

        segments.push(LoadedSegment {
            virtual_address: segment.p_vaddr,
            physical_address: address,
            memory_size: memory_size,
        });
    }

    Ok(segments)
}
