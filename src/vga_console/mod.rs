use core::ptr::Unique;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;
// const VGA_MEMORY_BASE: usize = 0xb8000;

#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Define a struct that represents a character on the screen.
// This uses a C-compatible representation to guarantee tha the
// ordering remains the same in memory (order is important here)

#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// Define a buffer – this is a two-dimensional array of characters that
// represent each addressable position on the screen.
struct Buffer {
    chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

// Now define a Writer – this stores the state of the screen (i.e. current
// colour code, position and buffer contents) and allows us to manipulate it
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

// Create a static Writer that we can use elsewhere
pub static mut WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) }
};

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }

                let row = SCREEN_HEIGHT - 1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.get_mut() }
    }

    fn new_line(&mut self) {/* TODO */}

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
    }
}

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let bytes = s.as_bytes();
        for byte in bytes {
          self.write_byte(*byte)
        }
        Ok(())
    }
}
