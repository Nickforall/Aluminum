use x86;

use spin::Mutex;
use x86::shared::dtables;
use x86::shared::dtables::DescriptorTablePointer;
use x86::bits64::irq::IdtEntry;

/// Creates an IDT entry.
///
/// Creates an IDT entry that executes the expression in `body`.
#[macro_export]
macro_rules! make_idt_entry {
    ($name:ident, $body:expr) => {{
        fn body() {
            $body
        }

        #[naked]
        unsafe extern fn $name() {
            asm!("push rbp
                  push r15
                  push r14
                  push r13
                  push r12
                  push r11
                  push r10
                  push r9
                  push r8
                  push rsi
                  push rdi
                  push rdx
                  push rcx
                  push rbx
                  push rax
                  mov rsi, rsp
                  push rsi
                  cli
                  call $0
                  sti
                  add rsp, 8
                  pop rax
                  pop rbx
                  pop rcx
                  pop rdx
                  pop rdi
                  pop rsi
                  pop r8
                  pop r9
                  pop r10
                  pop r11
                  pop r12
                  pop r13
                  pop r14
                  pop r15
                  pop rbp
                  iretq" :: "s"(body as fn()) :: "volatile", "intel");
                  ::core::intrinsics::unreachable();
        }

        use x86::shared::paging::VAddr;
        use x86::shared::PrivilegeLevel;
        use x86::bits64::irq::IdtEntry;

        let handler = VAddr::from_usize($name as usize);

        // last is "block", idk
        IdtEntry::new(handler, 0x8, PrivilegeLevel::Ring0, false)
    }};
}

/// The Interrupt Descriptor Table
///
/// The CPU will look at this table to find the appropriate interrupt handler.
static IDT: Mutex<[IdtEntry; 256]> = Mutex::new([IdtEntry::MISSING; 256]);

/// Pointer to the Interrupt Descriptor Table
pub struct IdtRef {
    ptr: DescriptorTablePointer<IdtEntry>,
    idt: &'static Mutex<[IdtEntry; 256]>,
}

unsafe impl Sync for IdtRef {}

impl IdtRef {
    /// Creates a new pointer struct to the IDT.
    pub fn new() -> IdtRef {
        let r = IdtRef {
            ptr: DescriptorTablePointer::new_idtp(&IDT.lock()[..]),
            idt: &IDT,
        };

        // This block is safe because by referencing IDT above, we know that we've constructed an
        // IDT.
        unsafe { dtables::lidt(&r.ptr) };

        r
    }

    /// Sets an IdtEntry as a handler for interrupt specified by `index`.
    pub fn set_handler(&self, index: usize, entry: IdtEntry) {
        self.idt.lock()[index] = entry;
    }

    /// Enables interrupts.
    pub fn enable_interrupts(&self) {
        // This unsafe fn is okay becuase, by virtue of having an IdtRef, we know that we have a
        // valid Idt.
        unsafe {
            x86::shared::irq::enable();
        }
    }
}