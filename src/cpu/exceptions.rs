use super::idt::IdtRef;
use super::pic;

pub fn register_exception_interrupts() {
    let divzerof = make_idt_entry!(isr0, {
        ::Context.load_error_screen();

        println!("Divided by zero!!!");
        unsafe { asm!("hlt") };

        pic::eoi_for(0);
    });

    let doublef = make_idt_entry!(isr8, {
        ::Context.load_error_screen();
        
        println!("Double Fault!!!");
        unsafe { asm!("hlt") };

        pic::eoi_for(0);
    });

    let gpf = make_idt_entry!(isr13, {
        println!("General protection fault!!!");
        pic::eoi_for(13);
    });

    let pf = make_idt_entry!(isr14, {
        println!("Page fault!!!");        
        pic::eoi_for(14);
    });
    
    ::Context.idt.set_handler(0, divzerof);
    ::Context.idt.set_handler(8, doublef);
    ::Context.idt.set_handler(13, gpf);
    ::Context.idt.set_handler(14, pf);
}