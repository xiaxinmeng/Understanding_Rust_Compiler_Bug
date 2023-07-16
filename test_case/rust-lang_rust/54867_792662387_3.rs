asm
reversed(unsigned __int128):
    mov     r8, rsi
    mov     r9, rdi
    mov     rsi, rdi
    mov     rax, r8
    mov     rdi, r8
    or      rax, r9
    je      .L6
    push    r15
    xor     eax, eax
    xor     edx, edx
    mov     r15d, 10
    push    r14
    movabs  r14, -3689348814741910324
    push    r13
    xor     r13d, r13d
    push    r12
    movabs  r12, -3689348814741910323
    push    rbp
    push    rbx
.L5:
    imul    rcx, rdx, 10
    mul     r15
    mov     r9, rdx
    mov     r8, rax
    add     r9, rcx
    mov     rcx, rsi
    add     rcx, rdi
    adc     rcx, 0
    xor     r11d, r11d
    mov     rax, rcx
    mul     r12
    mov     rax, rdx
    and     rdx, -4
    shr     rax, 2
    add     rdx, rax
    mov     rax, rsi
    sub     rcx, rdx
    mov     rdx, rdi
    sub     rax, rcx
    mov     r10, rcx
    sbb     rdx, r11
    mov     rbp, rdx
    mov     rdx, rax
    imul    rdx, r14
    imul    rbp, r12
    add     rbp, rdx
    mul     r12
    mov     rcx, rax
    mov     rbx, rdx
    and     eax, 1
    mov     edx, 5
    mul     rdx
    add     rbx, rbp
    add     rax, r10
    adc     rdx, r11
    add     rax, r8
    mov     r8, rdi
    mov     rdi, rbx
    adc     rdx, r9
    mov     r9, rsi
    mov     rsi, rcx
    shr     rdi
    shrd    rsi, rbx, 1
    mov     ebx, 9
    cmp     rbx, r9
    mov     rbx, r13
    sbb     rbx, r8
    jc      .L5
    pop     rbx
    pop     rbp
    pop     r12
    pop     r13
    pop     r14
    pop     r15
    ret
.L6:
    mov     rax, r9
    mov     rdx, r8
    ret
