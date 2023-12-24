#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kfs_v2::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Kernel Init starting...");
    kfs_v2::init();
    println!("Kernel Init done.");

    #[cfg(test)]
    test_main();

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
