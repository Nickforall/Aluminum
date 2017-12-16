global vga_print_error
global vga_clear

section .text
vga_print_error:
    ; call vga_clear

    mov dword [0xb8000], 'E' | (0xC | 0x0 << 4) << 8
    mov dword [0xb8002], 'R' | (0xC | 0x0 << 4) << 8
    mov dword [0xb8004], 'R' | (0xC | 0x0 << 4) << 8
    mov dword [0xb8006], 'O' | (0xC | 0x0 << 4) << 8
    mov dword [0xb8008], 'R' | (0xC | 0x0 << 4) << 8
    mov dword [0xb800a], edi
    hlt

vga_clear:
    mov eax, 0xb8000
.vga_loop:
    mov dword[eax], ' ' | (0x0 | 0x0 << 4) << 8 ; set the character to empty black on black
    add eax, 0x02   ; increase address by two bytes
    cmp eax, 0xb8fa0 ; compare it with the highest possible vga character in this case
    jle .vga_loop ; if we haven't reached above address yet, do it again

    ret