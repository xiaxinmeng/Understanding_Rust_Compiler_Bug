asm
reversed:
    push    rbp
    push    r15
    push    r14
    push    r13
    push    r12
    push    rbx
    sub     rsp, 8
    mov     rax, rdi
    or      rax, rsi
    je      .LBB0_1
    mov     rbx, rsi
    mov     r14, rdi
    xor     edx, edx
    mov     r15d, 10
    xor     ecx, ecx
.LBB0_4:
    mulx    rax, r13, r15
    lea     rcx, [rcx + 4*rcx]
    lea     r12, [rax + 2*rcx]
    mov     edx, 10
    mov     rdi, r14
    mov     rsi, rbx
    xor     ecx, ecx
    call    qword ptr [rip + __udivti3@GOTPCREL]
    mov     rsi, rdx
    mov     rdx, rax
    mulx    rcx, rdi, r15
    lea     rdx, [rsi + 4*rsi]
    lea     rbp, [rcx + 2*rdx]
    mov     rdx, r14
    sub     rdx, rdi
    mov     rcx, rbx
    sbb     rcx, rbp
    add     rdx, r13
    adc     rcx, r12
    cmp     r14, 10
    sbb     rbx, 0
    mov     r14, rax
    mov     rbx, rsi
    jae     .LBB0_4
    jmp     .LBB0_2
.LBB0_1:
    xor     edx, edx
    xor     ecx, ecx
.LBB0_2:
    mov     rax, rdx
    mov     rdx, rcx
    add     rsp, 8
    pop     rbx
    pop     r12
    pop     r13
    pop     r14
    pop     r15
    pop     rbp
    ret
