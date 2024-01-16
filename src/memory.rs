use crate::println;
use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

pub unsafe fn active_l4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (l4_table_frame, _) = Cr3::read();

    let phys = l4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

// Mayy be cool to print all mapped level 3 2 and 1 also
pub fn print_l4_table(phys_mem_offset: VirtAddr) {
    let l4_table = unsafe {active_l4_table(phys_mem_offset)};

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
}
