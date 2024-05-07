use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin::Mutex;

use crate::{println, drivers::{keyboard, timer}};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer as u8].set_handler_fn(timer::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard::keyboard_interrupt_handler);
        idt
    };
}

fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLEFAULT\n{:#?}", stack_frame)
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

fn init_pic() {
    unsafe {
        PICS.lock().initialize();
    }
}

pub fn init_intr() {
    init_idt();
    init_pic();
    x86_64::instructions::interrupts::enable();
}

pub fn notify_end_of_interrupt(interrupt_index: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_index as u8);
    }
}
