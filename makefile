add-toolchain:
	rustup component add rust-src --toolchain nightly

FORCE: ;

build-kernel: FORCE add-toolchain
	cargo +nightly build --release --target i386-target.json -Z build-std=core,alloc

kernel_entry.o:
	nasm kernel_entry.asm -f elf -o kernel_entry.o 

kernel.bin: kernel_entry.o build-kernel
	ld -m elf_i386 -o kernel.bin -Ttext 0x0 --oformat binary kernel_entry.o target/i386-target/release/deps/libos-*.a

boot_sect.bin: boot_sect.asm
	nasm -f bin $< -o $@ 

os-image.bin: boot_sect.bin kernel.bin
	cat $^ > $@

run: os-image.bin 
	qemu-system-i386 -fda $<