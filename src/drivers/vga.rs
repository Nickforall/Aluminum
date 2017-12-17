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
    pub fn new(enable_cursor: bool) -> Position {
        let pos = Position { x: 0, y: 0 };
        
        if enable_cursor {
            // pos.enable_cursor();
        } 

        pos
    }

    pub fn enable_cursor(&self) {
        // TODO: This one also doesn't work..
        unimplemented!();

        use x86::shared::io::{inb, outb};

        unsafe {
            outb(0x3D4, 0x0a);
            let start = inb(0x3d5) & 0x1f;

            outb(0x3d4, 0x0a);
            outb(0x3d5, start | 0x20);
        }
    }

    pub fn get_buffer_pos(&self, screen_width: u16) -> u16 {
        self.y * screen_width + self.x
    }

    pub fn update_cursor(&self, screen_width: u16) {
        // TODO: This one only worked for my first keyboard interrupt..., after that cursor disappears
        // Also, it was one x too high
        unimplemented!();   

        use x86::shared::io::outb;

        let pos = self.get_buffer_pos(screen_width);
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
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
            position: Position::new(true),
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

        // self.position.update_cursor(self.width);
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