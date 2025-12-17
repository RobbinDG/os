### Directories
BOOT_DIR=boot
BUILD_DIR=build

### Programs and arguments
GDB=gdb
QEMU=qemu-system-i386
LD_ARGS=-no-pie -nostdlib -m elf_i386 -T linker.ld
OBJDUMP_ARGS=--disassembler-color=on
LESS_ARGS=-R

add-toolchain:
	rustup component add rust-src --toolchain nightly

FORCE: ;

build-kernel: FORCE add-toolchain
	cargo build --release

$(BUILD_DIR)/%.o: $(BOOT_DIR)/%.asm
	nasm $< -g -f elf -o $@ 

$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel_entry.o build-kernel $(BUILD_DIR)/interrupt.o $(wildcard $(BOOT_DIR)/*/*.asm)
	ld $(LD_ARGS) -o $@ --oformat binary $(BUILD_DIR)/kernel_entry.o $(BUILD_DIR)/interrupt.o target/i386-target/release/deps/libos-*.a 

$(BUILD_DIR)/kernel.elf: $(BUILD_DIR)/kernel_entry.o build-kernel $(BUILD_DIR)/interrupt.o
	ld $(LD_ARGS) -o $@ $(BUILD_DIR)/kernel_entry.o $(BUILD_DIR)/interrupt.o target/i386-target/release/deps/libos-*.a 

$(BUILD_DIR)/boot_sect.bin: $(BOOT_DIR)/boot_sect.asm
	nasm -f bin $< -o $@ 

$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/boot_sect.bin $(BUILD_DIR)/kernel.bin
	cat $^ > $@

debug: $(BUILD_DIR)/os-image.bin $(BUILD_DIR)/kernel.elf
	$(QEMU) -no-reboot -s -fda $(BUILD_DIR)/os-image.bin &
	$(GDB) -ex "target remote localhost:1234" -ex "file $(BUILD_DIR)/kernel.elf"

run: $(BUILD_DIR)/os-image.bin 
	$(QEMU) -no-reboot -fda $< -boot order=ac


### OBJDUMPs

objdump-a: build-kernel
	objdump $(OBJDUMP_ARGS) -mi386 -d -C target/i386-target/release/deps/libos-*.a | less $(LESS_ARGS) -R

objdump-%.o: $(BUILD_DIR)/%.o
	objdump $(OBJDUMP_ARGS) -mi386 -d -C $^ | less $(LESS_ARGS) -R

hexdump-%.bin: $(BUILD_DIR)/%.bin
	hexdump -C $^ | less $(LESS_ARGS)

objdump-%.elf: $(BUILD_DIR)/%.elf
	objdump $(OBJDUMP_ARGS) -M i386,intel -D -C $^ | less $(LESS_ARGS)
