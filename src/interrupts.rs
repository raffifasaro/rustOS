use core::fmt::Debug;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

pub fn init_idt() {
    // initiating the ID Table for our exceptions
    let mut idt = InterruptDescriptorTable::new();
    // mapping out breakpoint handler fn to the breakpoint exception to our idt
    idt.breakpoint.set_handler_fn(breakpoint_handler);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // instead of temp pausing our program, we just print this message
    println!("BREAKPOINT EXCEPTION: \n{:#?}", stack_frame);
}