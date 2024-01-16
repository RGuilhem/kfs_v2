use x86_64::{PhysAddr, structures::paging::{PhysFrame, Size4KiB, OffsetPageTable, PageTable}, VirtAddr};
use bootloader::bootinfo::MemoryRegionType;
use x86_64::structures::paging::FrameAllocator;
use bootloader::bootinfo::MemoryMap;

pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let l4_table = active_l4_table(phys_mem_offset);
    OffsetPageTable::new(l4_table, phys_mem_offset)
}

unsafe fn active_l4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (l4_table_frame, _) = Cr3::read();

    let phys = l4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
