; 0x500 |       1 | nr of entries (N)
; 0x501 |   N x 1 | size of each entry (Si)
; 0x510 | SUM(Si) | entries, assume N is at most 15

read_high_mem:
    pusha
    ; Set ES:DI
    ; mov es, 1
    mov ax, 0
    mov es, ax
    mov di, 0x510 

    mov ebx, 0 ; clear ebx, start at first entry
    mov edx, 0x534d4150 ; set edx to magic number

    mov ax, 0 ; Use CX to store N

high_mem_loop:
    push ax
    mov eax, 0xe820 ; set eax to command
    mov ecx, 24
    int 0x15

    mov ch, 0
    add di, cx ; increment DI with # of read bytes

    ; EAX, ECX are OK to use until next iter
    pop ax
    push ebx
    add ax, 1 ; Increment count
    mov bx, ax
    mov [bx + 0x500], cl ; Store actual byte count

    ; 8 = FC00
    ; 10 = 0009
    ; 16 = 0010
    ; 20 = E987
    ; 22 = F000

    pop ebx
    cmp ebx, 0
    jne high_mem_loop

    sub ax, 1 ; Correct N for incorrect entry
    mov [0x500], ax

    popa
    ret