use core::ptr;

use uefi::boot::{self, AllocateType, MemoryType, PAGE_SIZE};
use uefi::Status;

use x86_64::structures::paging::PageTable;

/// Allocates a zeroed 4 KiB page for a page table.
pub fn allocate_page_table() -> Result<&'static mut PageTable, Status> {
    let page = boot::allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    )?;

    let table = page.as_ptr() as *mut PageTable;

    unsafe {
        ptr::write(table, PageTable::new());
        Ok(&mut *table)
    }
}

/// Returns the physical address of a page table.
pub fn physical_address(table: &PageTable) -> u64 {
    table as *const PageTable as u64
}
