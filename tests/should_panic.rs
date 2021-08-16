#![no_std]
#![no_main]


use core::panic::PanicInfo;
use f_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("\n[FAIL] [Test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Tests
fn should_fail() {
    serial_print!("\n[Panic test] ... ");
    assert_eq!(0, 1);
}

// Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[PASS]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
