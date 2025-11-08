[org 0x7c00] ; bootloader offset
KERNEL_OFFSET equ 0x1000

    mov [BOOT_DRIVE], dl ; BIOS stores boot drive # in dl at boot
    mov bp, 0x9000 ; set the stack
    mov sp, bp

    mov bx, MSG_REAL_MODE
    call print_str ; This will be written after the BIOS messages
    call print_nl

    call load_kernel
    call switch_to_pm
    jmp $ ; this will actually never be executed

%include "boot/16_bit/print_str.asm"
%include "boot/16_bit/print_hex.asm"
%include "boot/32_bit/gdt.asm"
%include "boot/32_bit/print_str.asm"
%include "boot/32_bit_switch.asm"
%include "boot/read_disk.asm"

[bits 16]
load_kernel:
    mov bx, MSG_LOAD_KERNEL
    call print_str
    call print_nl

    mov bx, KERNEL_OFFSET ; Read from disk and store in 0x1000
    mov al, 9 ; Load 8 code sectors and 1 rodata sector
    mov cl, 2 ; First available sector after boot sector
    mov dl, [BOOT_DRIVE]
    call read_disk 
    ret

[bits 32]
BEGIN_PM: ; after the switch we will get here
    mov ebx, MSG_PROT_MODE
    call print_string_pm ; Note that this will be written at the top left corner
    call KERNEL_OFFSET
    jmp $

BOOT_DRIVE db 0
MSG_REAL_MODE db "Started in 16-bit real mode", 0
MSG_PROT_MODE db "Loaded 32-bit protected mode", 0
MSG_LOAD_KERNEL db "Loading kernel into memory", 0

; bootsector
times 510-($-$$) db 0
dw 0xaa55