asm
example::test:
        push    rbp
        mov     rbp, rsp
        lea     rax, [rip + byte_str.0]
        cmp     rdi, rax
        je      .LBB0_1
        cmp     dword ptr [rdi], 875770417
        sete    al
        pop     rbp
        ret
.LBB0_1:
        mov     al, 1
        pop     rbp
        ret

byte_str.0:
        .ascii  "1234"
