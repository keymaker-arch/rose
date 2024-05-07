#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod drivers;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World: {}!", 1235);
    println!("Hello World: {}!", 1235);
    println!("Hello World: {}!", 1235);
    println!("Hello World: {}!", 1235);
    panic!();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    // println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}