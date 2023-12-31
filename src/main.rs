#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kfs_v2::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kfs_v2::init();
    println!("End of _start");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, flags) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );
    println!("Flags: {:?}", flags);

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
