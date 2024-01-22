#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use kfs_v2::allocator;
    use kfs_v2::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    kfs_v2::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    test_main();
    kfs_v2::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kfs_v2::test_panic_handler(info)
}
