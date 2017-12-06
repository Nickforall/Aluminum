arch ?= x86_64
target ?= $(arch)-aluminum
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

rust_os := target/$(target)/debug/libaluminum.a
linker_script := src/kernel/$(arch)/linker.ld
grub_cfg := src/kernel/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/kernel/$(arch)/*.asm)
assembly_object_files := $(patsubst src/kernel/$(arch)/%.asm, \
	build/kernel/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run debug iso cargo

all: $(kernel)

clean:
	@cargo clean
	@rm -rf build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s

debug: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s -d cpu_reset

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

cargo:
	@xargo build --target $(target)

# compile assembly files
build/kernel/$(arch)/%.o: src/kernel/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@