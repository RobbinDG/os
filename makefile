### Directories
BOOT_DIR=boot
BUILD_DIR=build
TARGET=target/i386-target/release/libos.a 
KERNEL_ASM=$(BUILD_DIR)/kernel_entry.o $(BUILD_DIR)/interrupt.o $(BUILD_DIR)/flush_segments.o

### Programs and arguments
GDB=gdb
GDBINIT=.gdbinit
QEMU=qemu-system-i386
LD_ARGS=-no-pie -nostdlib -m elf_i386 -T linker.ld
OBJDUMP_ARGS=--disassembler-color=on
LESS_ARGS=-R

.FORCE: ;

$(TARGET): .FORCE
	cargo build --release -Zjson-target-spec

test: .FORCE 
	cargo test --release -Zjson-target-spec


$(BUILD_DIR)/%.o: $(BOOT_DIR)/%.asm
	nasm $< -g -f elf -o $@ 

$(BUILD_DIR)/kernel.elf: $(KERNEL_ASM) $(TARGET) 
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
# $(QEMU) -no-reboot -s -fda $(BUILD_DIR)/os-image.bin -d int  &
	$(GDB) -ex "target remote localhost:1234"

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
