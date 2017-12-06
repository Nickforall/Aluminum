#![feature(lang_items)]
#![no_std]

mod drivers;

extern crate rlibc;

#[no_mangle]
pub extern fn kmain() {
    use drivers::vga;
    use drivers::vga::Color::{Black, LightGreen};

    let mut screen = vga::VgaScreen::defaults();
    screen.write("Welcome to, Aluminum Microkernel Experiment.");
}

#[lang = "eh_personality"] 
extern fn eh_personality() {
}

#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt() -> ! {
    loop {}
}