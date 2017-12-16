pub mod keymap;

pub struct Keyboard {
    shift: bool
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { shift: false }
    }

    pub fn get_char(&mut self, scancode: u8) -> char {
        if let Some(c) = keymap::US.get(scancode as usize) {
            if self.shift {
                c[1]
            } else {
                c[0]
            }
        } else {
            '\0'
        }
    }
}