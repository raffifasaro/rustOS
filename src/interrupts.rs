use core::fmt::Debug;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

// using lazy static to not run into lifetime issues with our idt
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        // initiating the ID Table for our exceptions
        let mut idt = InterruptDescriptorTable::new();
        // mapping out breakpoint handler fn to the breakpoint exception to our idt
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // load the idt with lidt using the x86 crate method
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // instead of temp pausing our program, we just print this message
    println!("BREAKPOINT EXCEPTION: \n{:#?}", stack_frame);
}
