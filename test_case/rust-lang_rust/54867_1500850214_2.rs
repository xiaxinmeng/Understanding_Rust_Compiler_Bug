asm
example::idiv10:
        push    rax
        mov     edx, 10
        xor     ecx, ecx
        call    qword ptr [rip + __divti3@GOTPCREL] ; clang: call __divti3@PLT
        pop     rcx
        ret
