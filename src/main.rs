#![no_std]
#![no_main]
// features for custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rustos::print;
use rustos::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rustos::test_panic_handler(_info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("RustOS running...");
    #[cfg(test)]
    test_main();

    loop {}
}
