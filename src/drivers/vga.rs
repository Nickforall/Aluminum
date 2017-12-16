use core::fmt;
use core::fmt::Write;

static VGA_ADDRESS: u32 = 0xb8000;

static VGA_WIDTH  : u16 = 80;
static VGA_HEIGHT : u16 = 24;

extern {
    fn vga_print_error(i: u16);
}

pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

pub fn make_entry(c: char, fg: Color, bg: Color) -> u16 {
    let color = (fg as u16) | (bg as u16) << 4;
    return (c as u16) | color << 8;
}

struct Position {
    x: u16,
    y: u16
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }
}

pub struct VgaScreen {
    width: u16,
    height: u16,
    address: u32,
    position: Position,
}

impl VgaScreen {
    pub fn defaults() -> Self {
        Self::new(VGA_WIDTH, VGA_HEIGHT, VGA_ADDRESS)
    }

    pub fn new(width: u16, height: u16, address: u32) -> Self {
        VgaScreen {
            width: width,
            height: height,
            address: address,
            position: Position::new(),
        }
    }

    pub fn newline(&mut self) {
        self.position.x = 0;
        self.position.y += 1;
    }

    pub unsafe fn putchar(&self, x: u16, y: u16, entry: u16) {
        if x >= VGA_WIDTH || y >= VGA_HEIGHT {
            return;
        }

        let offset = (y * VGA_WIDTH * 2 + x * 2) as u32;

        *((self.address + offset) as *mut u16) = entry;
    }

    /// The main way to write to the vga buffer, updates position and accepts strings (!)
    pub fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte as char == '\n' {
            self.newline();
            return;
        }

        if byte as char == '\0' {
            return;
        }

        unsafe {
            self.putchar(self.position.x, self.position.y, make_entry(
                byte as char,
                Color::LightGreen,
                Color::Black
            ));
        }

        self.position.x += 1;

        if self.position.x >= self.width {
            self.position.x = 0;
            self.position.y += 1;
        }
    }

    pub fn panic(&self, c: char) {
        unsafe {
            vga_print_error(make_entry(
                c,
                Color::LightRed,
                Color::Black
            ));
        }
    }
}

impl Write for VgaScreen {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}