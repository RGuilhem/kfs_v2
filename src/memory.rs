use x86_64::structures::paging::OffsetPageTable;
use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

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
