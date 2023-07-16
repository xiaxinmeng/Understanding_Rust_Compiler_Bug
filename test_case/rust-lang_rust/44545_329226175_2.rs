asm
divmod:                                 # @divmod
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        mov     r14, r8
        mov     rbx, rcx
        mov     r15, rdx
        mov     r12, rsi
        mov     r13, rdi

        mov     rdi, r12
        mov     rsi, r15
        mov     rdx, rbx
        mov     rcx, r14
        call    __udivti3
        mov     rcx, rax
        mov     rsi, rdx

        imul    r14, rcx
        mov     rax, rcx
        mul     rbx
        add     rdx, r14
        imul    rbx, rsi
        add     rbx, rdx
        sub     r12, rax
        sbb     r15, rbx

        mov     qword ptr [r13 + 8], rsi
        mov     qword ptr [r13], rcx
        mov     qword ptr [r13 + 16], r12
        mov     qword ptr [r13 + 24], r15

        mov     rax, r13
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        ret
