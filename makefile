### Directories
BOOT_DIR=boot
BUILD_DIR=build
TARGET=target/i386-target/release/libos.a 

### Programs and arguments
GDB=gdb
QEMU=qemu-system-i386
LD_ARGS=-no-pie -nostdlib -m elf_i386 -T linker.ld
OBJDUMP_ARGS=--disassembler-color=on
LESS_ARGS=-R

add-toolchain:
	rustup component add rust-src --toolchain nightly

.FORCE: ;

$(TARGET): .FORCE add-toolchain
	cargo build --release

$(BUILD_DIR)/%.o: $(BOOT_DIR)/%.asm
	nasm $< -g -f elf -o $@ 

$(BUILD_DIR)/kernel.elf: $(BUILD_DIR)/kernel_entry.o $(BUILD_DIR)/interrupt.o $(TARGET) 
	ld $(LD_ARGS) \
		--gc-sections \
		-Map=final.map \
		-o $@ \
		$^

$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel.elf
	objcopy -O binary $< $@

$(BUILD_DIR)/boot_sect.bin: $(BOOT_DIR)/boot_sect.asm
	nasm -f bin $< -o $@ 

$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/boot_sect.bin $(BUILD_DIR)/kernel.bin
	cat $^ > $@

clean:
	rm $(BUILD_DIR)/*
	rm -r target

debug: $(BUILD_DIR)/os-image.bin $(BUILD_DIR)/kernel.elf
	$(QEMU) -no-reboot -s -fda $(BUILD_DIR)/os-image.bin &
	$(GDB) -ex "target remote localhost:1234" -ex "file $(BUILD_DIR)/kernel.elf"

run: $(BUILD_DIR)/os-image.bin 
	$(QEMU) -no-reboot -fda $< -boot order=ac


### OBJDUMPs

objdump-a: $(TARGET)
	objdump $(OBJDUMP_ARGS) -mi386 -d -C $< | less $(LESS_ARGS)

objdump-%.o: $(BUILD_DIR)/%.o
	objdump $(OBJDUMP_ARGS) -mi386 -d -C $^ | less $(LESS_ARGS)

hexdump-%.bin: $(BUILD_DIR)/%.bin
	hexdump -C $^ | less $(LESS_ARGS)

objdump-%.elf: $(BUILD_DIR)/%.elf
	objdump $(OBJDUMP_ARGS) -M i386,intel -D -C $^ | less $(LESS_ARGS)
