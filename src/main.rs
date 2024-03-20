// compiler attribute
#![no_std]
// no main because we don't want to use the normal entry point chain (Rust runtime, start from crt0...)
#![no_main]

// basically import of PanicInfo
use core::panic::PanicInfo;

// Module for printing
mod vga_buffer;

// #[] are attributes that are used to mark stuff for the current crate
#[panic_handler]
// _ is convention because info parameter is not used
// -> ! specifies return type of function. ! is the "never type" indicating returning NOTHING, not even control to the caller
// (different to "void" equivalent of omitting the return type
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// overwrite of entry point
// no mangle to stop Rust from name mangling and outputs _start


static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
// "C" to use C calling convention
pub extern "C" fn _start() -> ! {
    // casting the int of our mem address into a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // iterating over bytes of HELLO string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // writing string byte and color byte
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

// DO NOT USE, WE HAVE OUR OWN TARGET SPECIFICATION
// --we need to build for a target triple with no underlying OS for now--
// --to build use: cargo build --target thumbv7em-none-eabihf--
// --(install target triple: rustup target add thumbv7em-none-eabihf)--

// we need nighly mode: rustup override set nightly


