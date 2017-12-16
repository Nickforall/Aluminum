extern set_up_page_tables
extern enable_paging
extern stack_top
extern vga_print_error
extern vga_clear
extern check_multiboot2
extern check_long_mode

extern kmain

section .text
global start

bits 32
start:
    mov esp, stack_top

    ; check whether we're loaded with multiboot2 compliance
    call check_multiboot2

    ; check whether this is a 64 bits cpu
    call check_long_mode

    ; setup paging and long mode
    call set_up_page_tables
    call enable_paging

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    jmp gdt64.code:long_mode_start

bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call vga_clear

    mov dword [0xb8000], 'O' | (0xa | 0x0 << 4) << 8
    mov dword [0xb8002], 'K' | (0xa | 0x0 << 4) << 8
    mov dword [0xb8004], 'A' | (0xa | 0x0 << 4) << 8
    mov dword [0xb8006], 'Y' | (0xa | 0x0 << 4) << 8
    mov dword [0xb8008], '!' | (0xa | 0x0 << 4) << 8

    call kmain

    hlt

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) ; data segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64