use x86_64::structures::idt::InterruptStackFrame;
use crate::{print, kernel::interrupt};

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    interrupt::notify_end_of_interrupt(interrupt::InterruptIndex::Timer);
}