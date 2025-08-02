#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}


fn should_fail() {
    serial_print!("should_panic::should fail... \t");
    assert_eq!(0, 1); // This will cause a panic
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}