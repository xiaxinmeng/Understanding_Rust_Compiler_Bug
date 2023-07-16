asm
example::foo:
        push    rbp
        mov     rbp, rsp
        mov     eax, 4
        pop     rbp
        ret
