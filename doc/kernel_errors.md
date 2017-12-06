# Kernel errors

Whenever a panic occurs on kernel level, the kernel will
try overriding the VGA buffer with the text 'ERROR', followed
by an identification number

## Using kernel panics

As a developer, you can trigger such a visual warning by moving
the vga entry hex code of the error code into `edx`, followed 
by calling `vga_print_error`.

## List of kernel panic identification numbers

* **`0` (0xc030)** - Initializing long mode was attempted on a cpu that does not support long mode
* **`1` (0xc031)** - The kernel was not loaded with a multiboot2 compliant loader