asm
example::test:
        lea     rax, [rip + byte_str.0]
        cmp     rdi, rax
        je      .LBB0_1
        push    rbp
        mov     rbp, rsp
        lea     rsi, [rip + byte_str.0]
        mov     edx, 3
        call    memcmp@PLT
        test    eax, eax
        sete    al
        pop     rbp
        ret
.LBB0_1:
        mov     al, 1
        ret

byte_str.0:
        .ascii  "123"
