#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[cfg(not(test))]
use crate::serial_print;

use volatile::Volatile;
use x86_64::instructions::port::Port;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    ascii_char: u8,
    color_code: ColorCode,
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<VgaChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buff: &'static mut VgaBuffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buff.chars[BUFFER_HEIGHT - 1][self.col_pos].write(VgaChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.col_pos += 1;
            }
        }
        self.update_cursor();
    }

    pub fn write_non_print(&mut self, byte: u8) {
        if byte == 8 {
            // handle backspace
            if self.col_pos > 0 {
                self.col_pos -= 1;
            }
            self.update_cursor();
            self.buff.chars[BUFFER_HEIGHT - 1][self.col_pos].write(VgaChar {
                ascii_char: b' ',
                color_code: self.color_code,
            });
        } else {
            self.write_byte(0xfe);
        }
    }

    fn update_cursor(&mut self) {
        let mut port_control = Port::new(0x3d4);
        let mut port_data = Port::new(0x3d5);
        let pos: u16 = ((BUFFER_HEIGHT - 1) * BUFFER_WIDTH + self.col_pos)
            .try_into()
            .expect("Vga cursor pos overflow");
        unsafe {
            port_control.write(0x0f as u8);
            port_data.write((pos & 0xff) as u8);
            port_control.write(0x0e as u8);
            port_data.write(((pos >> 8) & 0xff) as u8);
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buff.chars[row][col].read();
                self.buff.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.clear_dots();
        self.col_pos = 0;
    }

    fn clear_dots(&mut self) {
        let blank = VgaChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for i in 0..(BUFFER_HEIGHT - 2) {
            self.buff.chars[i][BUFFER_WIDTH - 1].write(blank);
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = VgaChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buff.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                non_print => self.write_non_print(non_print),
            }
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::DarkGray),
        buff: unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
    });
}

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
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
    #[cfg(not(test))]
    serial_print!("{}", args);
}

/// Called by the timer_interrupt_handler
///
/// Must not block or allocate
pub(crate) fn toggle_dot() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        // NOTE: not really sure without_interrupts needed
        let mut writer = WRITER.lock();
        let current = writer.buff.chars[BUFFER_HEIGHT - 1][BUFFER_WIDTH - 1].read();
        let color = writer.color_code;
        writer.buff.chars[BUFFER_HEIGHT - 1][BUFFER_WIDTH - 1].write(VgaChar {
            ascii_char: {
                if current.ascii_char == b' ' {
                    b'.'
                } else {
                    b' '
                }
            },
            color_code: color,
        });
    });
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buff.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_char), c);
        }
    });
}

#[test_case]
fn test_println_many_output() {
    for i in 0..300 {
        println!("One of many lines {}", i);
    }
}

#[test_case]
fn test_println_long_line() {
    println!(
        "A Very long line AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        klBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
        CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
        DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD"
    );
}
