asm
example::foo:
        push    rbp
        mov     rbp, rsp
        and     rsp, -268435456    # 0xfffffffff0000000
        mov     eax, 536870912    # 0x20000000
        call    __rust_probestack
        sub     rsp, rax
        mov     qword ptr [rsp], 2
        mov     rax, qword ptr [rsp]
        mov     rsp, rbp
        pop     rbp
        ret
