#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kfs_v2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kfs_v2::print;

//Testing environnement before calling init
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    kfs_v2::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kfs_v2::test_panic_handler(info)
}

#[test_case]
fn test_print() {
    print!("no crash without init\t");
}
