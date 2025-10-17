[org 0x7c00] ; bootloader offset
    mov bp, 0x9000 ; set the stack
    mov sp, bp

    mov bx, MSG_REAL_MODE
    call print_str ; This will be written after the BIOS messages

    call switch_to_pm
    jmp $ ; this will actually never be executed

%include "16_bit/print_str.asm"
%include "16_bit/print_hex.asm"
%include "32_bit/gdt.asm"
%include "32_bit/print_str.asm"
%include "32_bit_switch.asm"

[bits 32]
BEGIN_PM: ; after the switch we will get here
    mov ebx, MSG_PROT_MODE
    call print_string_pm ; Note that this will be written at the top left corner
    jmp $

MSG_REAL_MODE db "Started in 16-bit real mode", 0
MSG_PROT_MODE db "Loaded 32-bit protected mode", 0

; bootsector
times 510-($-$$) db 0
dw 0xaa55