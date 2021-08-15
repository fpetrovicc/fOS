// main.rs

#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Entry point
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Test panic message");
    loop{}
}

// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
