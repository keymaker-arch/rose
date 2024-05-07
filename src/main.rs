#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

mod drivers;
mod kernel;

use core::panic::PanicInfo;
use x86_64::instructions;

use drivers::tty;
use kernel::interrupt;

pub fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // tty::tty_init(tty::TTYOutputChannel::Serial0);
    tty::tty_init(tty::TTYOutputChannel::VGA);
    interrupt::init_intr();

    // x86_64::instructions::interrupts::int3();

    println!("Hello World!");
    // println!("Hello World: {}!", 9776);

    // #[cfg(test)]
    // {
    //     tty::tty_init(tty::TTYOutputChannel::Serial0);
    //     test_main();
    // }

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// #[test_case]
// fn trivial_assertion() {
//     print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }