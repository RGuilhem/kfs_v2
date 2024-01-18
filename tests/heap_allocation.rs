#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use kfs_v2::allocator::HEAP_SIZE;
use alloc::vec::Vec;
use alloc::boxed::Box;
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
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kfs_v2::test_panic_handler(info)
}

#[test_case]
fn simple_alloc() {
    let heap_val1 = Box::new(42);
    let heap_val2 = Box::new(24);
    assert_eq!(42, *heap_val1);
    assert_eq!(24, *heap_val2);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(i, *x);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(i, *x);
    }
    assert_eq!(1, *long_lived);
}
