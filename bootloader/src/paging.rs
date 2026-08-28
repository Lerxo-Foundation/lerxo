use x86_64::{
    PhysAddr,
    structures::paging::{
        FrameAllocator,
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
