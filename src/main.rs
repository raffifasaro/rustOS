// compiler attribute
#![no_std]
// no main because we don't want to use the normal entry point chain (Rust runtime, start from crt0...)
#![no_main]

// attributes for our custom tests
#![feature(custom_test_frameworks)]
#![test_runner(rustOS::test_runner)]

#![reexport_test_harness_main = "test_main"]

// "imports"
use core::panic::PanicInfo;
use rustOS::test_panic_handler;

// Module for printing
mod vga_buffer;
// module for serial port
mod serial;

// #[] are attributes that are used to mark stuff for the current crate
#[cfg(not(test))]
#[panic_handler]
// _ is convention because info parameter is not used
// -> ! specifies return type of function. ! is the "never type" indicating returning NOTHING, not even control to the caller
// (different to "void" equivalent of omitting the return type
fn panic(info: &PanicInfo) -> ! {
    // we use our println implementation to integrate the panic handler into our writer
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// overwrite of entry point
// no mangle to stop Rust from name mangling and outputs _start
#[no_mangle]
// "C" to use C calling convention
pub extern "C" fn _start() -> ! {
    // use of our write_str implementation
    //vga_buffer::WRITER.lock().write_str("Amogus").unwrap();

    // use of our println implementation
    println!("Amogus");

    #[cfg(test)]
    test_main();

    loop {}
}


// DO NOT USE, WE HAVE OUR OWN TARGET SPECIFICATION
// --we need to build for a target triple with no underlying OS for now--
// --to build use: cargo build --target thumbv7em-none-eabihf--
// --(install target triple: rustup target add thumbv7em-none-eabihf)--

// we need nighly mode: rustup override set nightly


