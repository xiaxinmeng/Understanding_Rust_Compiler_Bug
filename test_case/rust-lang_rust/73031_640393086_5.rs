assembly
example::foo:
        push    rax
        xor     r8d, r8d
        cmp     esi, 5
        sete    dl
        mov     cl, 2
        sub     cl, dl
        mov     byte ptr [rdi], cl
        mov     eax, 1
        cmp     cl, 1
        je      .LBB5_3
        mov     r8b, dl
        mov     eax, 2
        sub     rax, r8
        cmp     rax, 2
        jne     .LBB5_4
        mov     eax, 2
.LBB5_3:
        pop     rcx
        ret
.LBB5_4:
        call    std::panicking::begin_panic
        ud2
