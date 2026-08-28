use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned, MemoryType};
use x86_64::{
    PhysAddr,
    structures::paging::{
        FrameAllocator,
        PhysFrame,
        Size4KiB,
    },
};

pub struct BootFrameAllocator {
    regions: MemoryMapOwned,
    region_index: usize,
    next_frame: u64,
}

impl BootFrameAllocator {
    pub fn new(memory_map: MemoryMapOwned) -> Self {
        Self {
            regions: memory_map,
            region_index: 0,
            next_frame: 0,
        }
    }

    fn find_next_usable_region(&mut self) -> Option<()> {
        while self.region_index < self.regions.len() {
            let region = self.regions.get(self.region_index)?;

            if region.ty == MemoryType::CONVENTIONAL
                && region.page_count > 0
            {
                self.next_frame = region.phys_start;
                return Some(());
            }

            self.region_index += 1;
        }

        None
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        loop {
            if self.region_index >= self.regions.len() {
                return None;
            }

            let region = self.regions.get(self.region_index)?;

            if region.ty != MemoryType::CONVENTIONAL
                || region.page_count == 0
            {
                self.region_index += 1;
                continue;
            }

            let region_end =
                region.phys_start + region.page_count * 4096;

            if self.next_frame == 0 {
                self.next_frame = region.phys_start;
            }

            if self.next_frame >= region_end {
                self.region_index += 1;
                self.next_frame = 0;
                continue;
            }

            let frame = PhysFrame::from_start_address(
                PhysAddr::new(self.next_frame),
            )
            .ok()?;

            self.next_frame += Size4KiB::SIZE;

            return Some(frame);
        }
    }
}
