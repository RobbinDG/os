global flush_segments

flush_segments:
    jmp 0x08:flush_data_segments

flush_data_segments:
    mov ax, 0x10 
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret