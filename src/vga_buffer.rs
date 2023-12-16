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

#[repr(transparent)]
struct VgaBuffer {
    chars: [[VgaChar; BUFFER_WIDTH]; BUFFER_HEIGHT];
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
                if (self.col_pos >= BUFFER_WIDTH) {
                    self.new_line();
                }
                self.buff[BUFFER_HEIGHT - 1][self.col_pos] = VgaChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                }
                self.col_pos += 1;
            }
        }
    }

    pub fn new_line(&mut self) {}

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}
