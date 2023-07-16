asm
<example::O as example::A>::k:
        mov     eax, esi
        ret

<example::P as example::A>::k:
        lea     eax, [rsi + 10]
        ret

example::R::k:
        lea     eax, [rsi + 10]
        cmp     byte ptr [rdi], 1
        cmove   eax, esi
        ret

example::R::j:
        mov     rax, rdi
        add     rdi, 1
        cmp     byte ptr [rax], 1
        jne     .LBB3_1
        mov     rax, qword ptr [rip + <example::O as example::A>::k@GOTPCREL]
        jmp     rax
.LBB3_1:
        mov     rax, qword ptr [rip + <example::P as example::A>::k@GOTPCREL]
        jmp     rax
