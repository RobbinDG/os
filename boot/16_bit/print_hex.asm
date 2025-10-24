make_numeric:
    add cl, '0'
    jmp make_done

make_alpha:
    add cl, 'A' - 10
    jmp make_done

print_hex_digit:
    cmp cl, 0x09
    jg make_alpha
    jmp make_numeric
make_done:
    mov al, cl
    int 0x10
    ret
    
; in: dx = value
print_hex:
    pusha

    mov ah, 0x0e ; tty mode

    mov al, '0'
    int 0x10
    mov al, 'x'
    int 0x10

    mov cl, dh
    and cl, 0xF0
    shr cl, 4
    call print_hex_digit
    mov cl, dh
    and cl, 0x0F
    call print_hex_digit
    mov cl, dl
    and cl, 0xF0
    shr cl, 4
    call print_hex_digit
    mov cl, dl
    and cl, 0x0F
    call print_hex_digit

    popa
    ret