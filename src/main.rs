#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use kfs_v2::println;

entry_point!(kernel_main);

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Start of _start");
    kfs_v2::init();

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
