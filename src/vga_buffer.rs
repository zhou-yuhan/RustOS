use core::fmt;
use volatile::Volatile;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LighGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(front: Color, background: Color) -> ColorCode {
        ColorCode(((background as u8) << 4) | (front as u8))
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ScreenCharactor {
    // VGA screen charactor, only supports ASCII
    charactor: u8,
    color_code: ColorCode,
}

const VGA_BUFFER_W: usize = 80;
const VGA_BUFFER_H: usize = 25;

type VgaBuffer = [[Volatile<ScreenCharactor>; VGA_BUFFER_W]; VGA_BUFFER_H];

pub struct VgaWriter {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut VgaBuffer,
}

impl VgaWriter {
    pub fn new(front: Color, background: Color) -> VgaWriter {
        VgaWriter {
            col_pos: 0,
            color_code: ColorCode::new(front, background),
            // raw pointer points to 0xb8000 in RAM
            // where VGA device is mapped
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= VGA_BUFFER_W {
                    self.new_line()
                }

                let row = VGA_BUFFER_H - 1; // TODO: only print bytes at bottom
                let col = self.col_pos;
                self.buffer[row][col].write(ScreenCharactor {
                    charactor: byte,
                    color_code: self.color_code,
                });
                self.col_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // unprintable charactor ︎
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_H {
            for col in 0..VGA_BUFFER_W {
                let screen_char = self.buffer[row][col].read();
                self.buffer[row - 1][col].write(screen_char);
            }
        }
        self.clear_row(VGA_BUFFER_H - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_BUFFER_W {
            self.buffer[row][col].write(ScreenCharactor {
                charactor: b' ',
                color_code: ColorCode::new(Color::Black, Color::Black),
            });
        }
    }
}

impl fmt::Write for VgaWriter {
    // use core::fmt to format output string
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// initialize global VGA writer with non-static functions (deref 0xb8000 etc.)
use lazy_static::lazy_static;
// use spinlock to synchronize concurrent writes to VGA device
use spin::Mutex;

lazy_static! {
    pub static ref VGA_WRITER: Mutex<VgaWriter> = Mutex::new(
        VgaWriter::new(Color::Green, Color::Black)
    );
}

// from std println! source
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}