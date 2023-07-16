asm
divmod:                                 # @divmod
        push    rbp
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        push    rax
        mov     r14, r8
        mov     r15, rcx
        mov     r12, rdx
        mov     r13, rsi
        mov     rbx, rdi

        mov     rdi, r13
        mov     rsi, r12
        mov     rdx, r15
        mov     rcx, r14
        call    __udivti3
        mov     qword ptr [rsp], rax    # 8-byte Spill
        mov     rbp, rdx

        mov     rdi, r13
        mov     rsi, r12
        mov     rdx, r15
        mov     rcx, r14
        call    __umodti3

        mov     qword ptr [rbx + 8], rbp
        mov     rcx, qword ptr [rsp]    # 8-byte Reload
        mov     qword ptr [rbx], rcx
        mov     qword ptr [rbx + 24], rdx
        mov     qword ptr [rbx + 16], rax

        mov     rax, rbx
        add     rsp, 8
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        pop     rbp
        ret
