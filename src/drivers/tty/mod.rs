mod vga;
mod serial;

use spin::Mutex;

static TTY_OUTPUT_CHANNEL: Mutex<TTYOutputChannel> = Mutex::new(TTYOutputChannel::VGA);

pub enum TTYOutputChannel {
    VGA,
    Serial0,
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        match *TTY_OUTPUT_CHANNEL.lock() {
            TTYOutputChannel::VGA => vga::VGA_SCREEN.lock().write_fmt(args).unwrap(),
            TTYOutputChannel::Serial0 => serial::SERIAL_PORT0.lock().write_fmt(args).unwrap(),
        }
    })
}

pub fn tty_init(channel: TTYOutputChannel) {
    *TTY_OUTPUT_CHANNEL.lock() = channel;
}