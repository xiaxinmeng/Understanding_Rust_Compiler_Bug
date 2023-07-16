asm
example::foo:
        push    rbp
        mov     eax, 536870896    # 0x1ffffff0
        call    __chkstk
        sub     rsp, rax
        lea     rbp, [rsp + 128]
        and     rsp, -268435456    # 0xfffffffff0000000
        mov     qword ptr [rsp], 2
        mov     rax, qword ptr [rsp]
        lea     rsp, [rbp + 536870768]    # 0x1fffff70
        pop     rbp
        ret
