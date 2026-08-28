use alloc::vec::Vec;
use core::ptr;

use uefi::boot::{self, AllocateType, MemoryType, PAGE_SIZE};
use uefi::Status;

use crate::elf::KernelElf;

pub struct LoadedSegment {
    pub virtual_address: u64,
    pub physical_address: usize,
    pub memory_size: usize,
}

pub fn load_segments(
    kernel: &KernelElf<'_>,
) -> Result<Vec<LoadedSegment>, Status> {
    let mut segments = Vec::new();

    for segment in kernel.loadable_segments() {
        let file_size = segment.p_filesz as usize;
        let memory_size = segment.p_memsz as usize;
        let file_offset = segment.p_offset as usize;

        if file_size > memory_size {
            return Err(Status::LOAD_ERROR);
        }

        let file_end = file_offset
            .checked_add(file_size)
            .ok_or(Status::LOAD_ERROR)?;

        if file_end > kernel.data().len() {
            return Err(Status::LOAD_ERROR);
        }

        if memory_size == 0 {
            continue;
        }

        let pages = memory_size.div_ceil(PAGE_SIZE);

        let destination = boot::allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            pages,
        )?;

        let physical_address = destination.as_ptr() as usize;

        unsafe {
            let destination_slice =
                core::slice::from_raw_parts_mut(
                    destination.as_ptr(),
                    memory_size,
                );

            let source = &kernel.data()[file_offset..file_end];

            destination_slice[..file_size]
                .copy_from_slice(source);

            ptr::write_bytes(
                destination_slice[file_size..].as_mut_ptr(),
                0,
                memory_size - file_size,
            );
        }

        segments.push(LoadedSegment {
            virtual_address: segment.p_vaddr,
            physical_address,
            memory_size,
        });
    }

    Ok(segments)
}
