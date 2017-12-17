#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]

#[macro_use]
mod macros;

mod drivers;
#[macro_use]
mod cpu;
mod kernel;

extern crate rlibc;
extern crate x86;
extern crate spin;
#[macro_use]
extern crate lazy_static;

use x86::shared::io::{inb};
use kernel::KernelContext;

lazy_static! {
    pub static ref Context: KernelContext = KernelContext::new();
}

#[no_mangle]
pub extern fn kmain() {
    //let mut screen = Context.vga;
    //screen.write("Welcome to, Nick's Aluminum Microkernel Experiment (v0.1).\n");
    println!("Welcome to, Nick's Aluminum Microkernel Experiment (v0.1).");

    cpu::pic::remap();
    println!("PIC initialized.");

    let timer = make_idt_entry!(isr32, {
        cpu::pic::eoi_for(32);
    });

    let kb = make_idt_entry!(isr33, {
        unsafe { 
            let x = inb(0x60); 
            let mut kb = Context.keyboard.lock();
            // kb.get_char(str::from_utf8(x))
            use ::core::str;

            let c = kb.get_char(x) as u8;

            print!("{}", str::from_utf8(&[c]).unwrap());
        };

        // used for testing exception handlers
        /*unsafe {
            asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
        }*/


        cpu::pic::eoi_for(60);
    });

    Context.idt.set_handler(32, timer);
    Context.idt.set_handler(33, kb);

    cpu::exceptions::register_exception_interrupts();

    Context.idt.enable_interrupts();

    println!("Interrupts enabled.");

    println!("Aluminum has booted.");
    

    loop {};
}

#[lang = "eh_personality"] 
extern fn eh_personality() {
}

#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt() -> ! {
    loop {}
}