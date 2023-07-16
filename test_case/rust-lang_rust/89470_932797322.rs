asm
example::without_move:
        sub     rsp, 24
        mov     qword ptr [rsp + 8], rdi
        lea     rax, [rsp + 8]
        mov     qword ptr [rsp + 16], rax
        lea     rsi, [rip + .L__unnamed_2]
        lea     rdi, [rsp + 16]
        call    qword ptr [rip + use_it@GOTPCREL]
        add     rsp, 24
        ret
