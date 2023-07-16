asm
<example::O as example::A>::k:
        mov     eax, esi
        ret

example::R::k:
        mov     eax, esi
        ret

example::R::j:
        add     rdi, 1
        jmp     qword ptr [rip + <example::O as example::A>::k@GOTPCREL]
