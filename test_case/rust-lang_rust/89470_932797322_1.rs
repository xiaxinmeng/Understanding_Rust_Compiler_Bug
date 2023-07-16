asm
example::without_move:
        sub     rsp, 40
        lea     rax, [rdi + 4]
        lea     rcx, [rdi + 8]
        mov     qword ptr [rsp + 8], rdi
        add     rdi, 12
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 24], rcx
        mov     qword ptr [rsp + 32], rdi
        lea     rsi, [rip + .L__unnamed_2]
        lea     rdi, [rsp + 8]
        call    qword ptr [rip + use_it@GOTPCREL]
        add     rsp, 40
        ret
