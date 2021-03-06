// main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(f_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use f_os::println;

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[fOS ENTRY POINT LOADED]");
    
    #[cfg(test)]
    test_main();

    loop{}
}

// Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    f_os::test_panic_handler(info)
}
