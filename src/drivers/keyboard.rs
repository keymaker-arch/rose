use x86_64::{structures::idt::InterruptStackFrame, instructions::port::Port};
use pc_keyboard::{ScancodeSet1, layouts::Us104Key, DecodedKey, HandleControl, Keyboard};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{print, kernel::interrupt};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore));
    static ref KEYBOARD_PORT: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { KEYBOARD_PORT.lock().read() };
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    interrupt::notify_end_of_interrupt(interrupt::InterruptIndex::Keyboard);
}