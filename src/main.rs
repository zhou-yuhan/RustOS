#![no_std]
#![no_main]
// features for custom test framework
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Color, VGA_WRITER};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

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

#[cfg(test)]
fn test_runner(tests: &[&dyn Test]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // exit_qemu(QemuExit::Success);
}

#[test_case]
fn dummy_test() {
    assert_eq!(42, 42);
}

#[test_case]
fn vga_test() {
    println!("First line to be tested");
    println!(
        "Test {} + {} = {} and {} / {} = {}",
        3,
        2,
        3 + 2,
        3,
        2,
        3.0 / 2.0
    );
    println!("😂🧐🤩");
    for i in 0x20u8..=0x7eu8 {
        print!("{}", i as char);
    }
}
