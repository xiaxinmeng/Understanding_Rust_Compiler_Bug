asm
example::func:
        push    r15
        push    r14
        push    r12
        push    rbx
        mov     eax, 300024
        call    __rust_probestack
        sub     rsp, rax // <-- reserve 300k of stack space
        ...
