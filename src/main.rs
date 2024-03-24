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

// static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
// "C" to use C calling convention
pub extern "C" fn _start() -> ! {
    // casting the int of our mem address into a raw pointer
    // let vga_buffer = 0xb8000 as *mut u8;

    // iterating over bytes of HELLO string
    /*
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // writing string byte and color byte
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */

    // use of our write_str implementation
    //vga_buffer::WRITER.lock().write_str("Amogus").unwrap();

    // use of our println implementation
    println!("Amogus");

    #[cfg(test)]
    test_main();

    loop {}
}



// this part has been moved to the lib
/*
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        // creating port at the isa-debug-exit
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}


// we use cfg(test) to only include the test runner in tests
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("{} tests running", tests.len());
    for test in tests {
        test.run();
    }

    // exit qemu after all tests
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}
// implementing testable to print the error message for tests
impl<T> Testable for T where T: Fn(), {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}

// tests
// test case to check for panic on print
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

// qemu specific exit function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
*/

// DO NOT USE, WE HAVE OUR OWN TARGET SPECIFICATION
// --we need to build for a target triple with no underlying OS for now--
// --to build use: cargo build --target thumbv7em-none-eabihf--
// --(install target triple: rustup target add thumbv7em-none-eabihf)--

// we need nighly mode: rustup override set nightly


