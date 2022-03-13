#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Color, VgaWriter};
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = VgaWriter::new(Color::Green, Color::DarkGray);
    writer.write_byte(b'H');
    writer.write_string("ello world\nThis is good\n");

    write!(writer, "This line calculates {} / {} = {}\n", 1, 3, 1 / 3).unwrap();

    let msg = "OK";
    println!("This line is from println{} macro, {}", "!", msg);

    loop {}
}
