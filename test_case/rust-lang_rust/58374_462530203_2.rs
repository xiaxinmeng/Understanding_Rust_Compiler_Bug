asm
example::foo:
        push    rax
        mov     dword ptr [rsp], 100
        pop     rax
        ret
