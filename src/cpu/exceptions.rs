use super::idt::IdtRef;
use super::pic;

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub fn register_exception_interrupts() {
    let divzerof = make_idt_entry!(isr0, {
        ::Context.load_error_screen();

        let stack_frame: &ExceptionStackFrame;
        unsafe {
            asm!("mov $0, rsp" : "=r"(stack_frame) ::: "intel");
        }

        println!("Divided by zero!!!\n{:#?}", stack_frame);
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