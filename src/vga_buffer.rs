// silence compiler for unused variants
#[allow(dead_code)]
// enable copy semantics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store each as u8
#[repr(u8)]
// enum of colors
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    // constructor taking foreground and background color and appending them to an u8
    // (The << 4 is important because background color only uses 3 bits, the last bit is to indicate flashing behaviour)
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// repr(C) guarantees field ordering
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    // current position
    column_position: usize,
    // foreground and background color
    color_code: ColorCode,
    // reference to VGA buffer
    buffer: &'static mut Buffer,
}

impl Writer {
    fn new_line(&mut self) {
        // todo
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            //if match is byte literal execute newline()
            b'\n' => self.new_line(),
            byte => {
                // check if current line is full
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;
                let color_code = self.color_code;

                // write new ScreenChar to current position
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                // advance column position when done
                self.column_position += 1;

            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // if the byte is Ascii printable or newline, write
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // else print ■
                _ => self.write_byte(0xfe),
            }
        }
    }
}


// temp write function
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Cyan, Color::DarkGray),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
    };

    writer.write_string("Test");
}














