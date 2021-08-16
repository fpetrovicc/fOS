// main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// Entry point
#[no_mangle]
pub extern "C" fn _start() {
    println!("[fOS entry point loaded]");
    
    #[cfg(test)]
    test_main();

    loop{}
}

// Function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\n[Running {} tests]", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("[{}] ... ", core::any::type_name::<T>());
        self();
        serial_println!("[PASS]");
    }
}

// Test cases
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1)
}

// Panic test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAIL]\n");
    serial_println!("[ERROR] ... [{}]\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop{}
}

// Exit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
