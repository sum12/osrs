#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osrs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osrs::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello through macro {} in main.rs", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osrs::serial_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osrs::test_panic_handler(info)
}
