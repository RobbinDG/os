[bits 32]
[extern kernel_main]
section .text.kernel_entry
    global kernel_entry
kernel_entry:
    call kernel_main
    jmp $