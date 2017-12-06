extern vga_print_error

global check_multiboot2
global check_long_mode

section .text

check_multiboot2:
    mov edi, 0xc031

    ; if we're booted with a multiboot2 compliant loader, the magic byte will be in eax
    cmp eax, 0x36d76289
    jne vga_print_error

    ret

check_long_mode:
    ; extended cpu info
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29 ; long mode bit
    mov edi, 0xc030 ; long mode error code
    jz vga_print_error ; error if long mode is zero

    ret

