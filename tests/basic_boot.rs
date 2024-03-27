
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustOS::{init, test_panic_handler};
use rustOS::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_breakpoint_exception() {
    // testing the breakpoint exception handler
    x86_64::instructions::interrupts::int3();
}