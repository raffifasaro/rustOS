//compiler attribute
#![no_std]
//no main because we don't want to use the normal entry point chain (Rust runtime, start from crt0...)
#![no_main]

//basically import of PanicInfo
use core::panic::PanicInfo;

//#[] are attributes that are used to mark stuff for the current crate
#[panic_handler]
//_ is convention because info parameter is not used
//-> ! specifies return type of function. ! is the "never type" indicating returning NOTHING, not even control to the caller
// (different to "void" equivalent of omitting the return type
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//overwrite of entry point
//no mangle to stop Rust from name mangling and outputs _start
#[no_mangle]
// "C" to use C calling convention
pub extern "C" fn _start() -> ! {
    loop {}
}
// we need to build for a target triple with no underlying OS for now
// --to build use: cargo build --target thumbv7em-none-eabihf--
// --(install target triple: rustup target add thumbv7em-none-eabihf)--

// we need nighly mode: rustup override set nightly

//Self defined tt: cargo build --target x86_64-RustOStt.json