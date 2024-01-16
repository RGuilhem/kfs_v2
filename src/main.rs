#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use kfs_v2::memory;
use x86_64::VirtAddr;
use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use kfs_v2::println;
use alloc::boxed::Box;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use crate::memory::BootInfoFrameAllocator;
    use kfs_v2::allocator;
    println!("Start of _start");
    kfs_v2::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    let test_x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("End of _start");
    kfs_v2::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    kfs_v2::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kfs_v2::test_panic_handler(_info)
}
