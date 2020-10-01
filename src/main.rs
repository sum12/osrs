#![no_std]
#![no_main]

extern crate rlibc;
mod vga_buffer;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello through macro {}", "!");
    panic!("Dang it !!!");
}

