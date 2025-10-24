BOOT_DIR=boot
BUILD_DIR=build

add-toolchain:
	rustup component add rust-src --toolchain nightly

FORCE: ;

build-kernel: FORCE add-toolchain
	cargo +nightly build --release --target i386-target.json -Z build-std=core,alloc

$(BUILD_DIR)/%.o: $(BOOT_DIR)/%.asm
	nasm $< -f elf -o $@ 

$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel_entry.o build-kernel
	ld -m elf_i386 -o $@ -Ttext 0x0 --oformat binary kernel_entry.o target/i386-target/release/deps/libos-*.a

$(BUILD_DIR)/boot_sect.bin: $(BOOT_DIR)/boot_sect.asm
	nasm -f bin $< -o $@ 

$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/boot_sect.bin $(BUILD_DIR)/kernel.bin
	cat $^ > $@

run: $(BUILD_DIR)/os-image.bin 
	qemu-system-i386 -fda $<