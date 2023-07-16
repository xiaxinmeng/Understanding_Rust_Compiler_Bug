asm
example::xx:
        push    rbx
        mov     rbx, rdi
        call    qword ptr [rsi + 24]
        mov     rax, rbx
        pop     rbx
        ret
