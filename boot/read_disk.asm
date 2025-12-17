; in
; - al: number of sectors
; - cl: sector number
; - es:bx: buffer start
read_disk:
    pusha

    push ax ; back up sector count

    mov ah, 0x02
    mov ch, 0 ; track/cyl number
    mov dh, 0 ; head number
    ; mov dl, 0 ; drive number, set by BIOS

    int 0x13

    mov bl, al ; move actual sector count
    pop ax

    jc disk_error ; error in carry bit

disk_done:
    popa
    ret

disk_error:
    cmp bl, al 
    jl too_few_sectors
    cmp ah, 0
    jg disk_status
    jmp disk_done

too_few_sectors:
    mov bx, TOO_FEW_SECTORS_MSG
    call print_str 
    call print_nl
    jmp disk_done

disk_status:
    mov bx, DISK_ERROR_STATUS_MSG
    call print_str 
    mov dl, ah
    mov dh, 0
    call print_hex 
    jmp disk_done


TOO_FEW_SECTORS_MSG:
    db 'Too few sectors', 0

DISK_ERROR_STATUS_MSG:
    db 'Disk Err: ', 0

BEFORE:
    db 'B', 0
AFTER:
    db 'A', 0