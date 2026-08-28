use x86_64::{
    VirtAddr,
    PhysAddr,
    structures::paging::{
        FrameAllocator,
        OffsetPageTable,
        PageTable,
        PhysFrame,
        Size4KiB,
    },
};

pub struct BootFrameAllocator {
    next: u64,
    end: u64,
}

impl BootFrameAllocator {
    pub fn new(start: u64, end: u64) -> Self {
        assert!(start % Size4KiB::SIZE == 0);
        assert!(end % Size4KiB::SIZE == 0);
        assert!(start <= end);

        Self {
            next: start,
            end,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if self.next >= self.end {
            return None;
        }

        let frame = PhysFrame::from_start_address(
            PhysAddr::new(self.next),
        )
        .ok()?;

        self.next += Size4KiB::SIZE;

        Some(frame)
    }
}

/// Creates a mapper for an existing x86-64 page-table hierarchy.
///
/// `phys_offset` must correspond to a virtual mapping of all physical
/// memory. The caller must also provide the currently active level-4 table.
pub unsafe fn create_mapper(
    level_4_table: &'static mut PageTable,
    phys_offset: VirtAddr,
) -> OffsetPageTable<'static> {
    OffsetPageTable::new(level_4_table, phys_offset)
}
