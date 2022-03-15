#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Color, VGA_WRITER};


#[repr(u32)]
pub enum QemuExit {
    Success = 0x10,
    Failed = 0x11,
}

#[allow(dead_code)]
fn exit_qemu(code: QemuExit) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4); // x86_64 iobase
        port.write(code as u32);
    }
}

pub trait Test {
    fn run(&self) -> ();
}

impl<T: Fn()> Test for T {
    fn run(&self) -> () {
        println!("running {}...", core::any::type_name::<T>());
        self();
        println!("");
        VGA_WRITER.lock().change_color(Color::Green, Color::Black);
        println!("[PASS]");
        VGA_WRITER.lock().change_color(Color::White, Color::Black);
    }
}

pub fn test_runner(tests: &[&dyn Test]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // exit_qemu(QemuExit::Success);
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    VGA_WRITER.lock().change_color();
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_panic_handler(_info);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}