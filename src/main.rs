
#![no_std]

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

fn main() {}