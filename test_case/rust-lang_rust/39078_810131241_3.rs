asm
foo(unsigned __int128):
    mov     r9, rdi
    mov     r8, rsi
    mov     rsi, rdi
    mov     rcx, r9
    mov     rdi, r8
    add     rcx, r8
    movabs  r8, -3689348814741910323
    adc     rcx, 0
    xor     r11d, r11d
    mov     rax, rcx
    mul     r8
    mov     rax, rdx
    and     rdx, -4
    shr     rax, 2
    add     rdx, rax
    mov     rax, r9
    sub     rcx, rdx
    mov     rdx, rdi
    sub     rax, rcx
    sbb     rdx, r11
    mov     r11d, 10
    mov     rcx, rdx
    movabs  rdx, -3689348814741910324
    imul    rcx, r8
    imul    rdx, rax
    add     rcx, rdx
    mul     r8
    add     rdx, rcx
    shrd    rax, rdx, 1
    mul     r11
    sub     rsi, rax
    mov     eax, esi
    ret
