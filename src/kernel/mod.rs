use cpu::idt;
use drivers::ps2;
use drivers::vga;

use spin::Mutex;

pub struct KernelContext {
    pub idt: idt::IdtRef,
    pub keyboard: Mutex<ps2::Keyboard>,
    pub vga: Mutex<vga::VgaScreen>
}

impl KernelContext {
    pub fn new() -> Self {
        KernelContext {
            idt: idt::IdtRef::new(),
            keyboard: Mutex::new(ps2::Keyboard::new()),
            vga: Mutex::new(vga::VgaScreen::defaults()),
        }
    }

    pub fn load_error_screen(&self) {
        let mut screen = self.vga.lock();
        screen.set_color(vga::Color::White, vga::Color::Red);
        screen.fill(vga::Color::White, vga::Color::Red);
    }
}