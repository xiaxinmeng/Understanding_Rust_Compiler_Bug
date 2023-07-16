assembly
example::vec_cast:
        mov     rax, rdi
        mov     rcx, qword ptr [rsi]
        mov     rdx, qword ptr [rsi + 8]
        mov     rdi, qword ptr [rsi + 16]
        mov     rsi, rcx
        test    rdi, rdi
        je      .LBB0_2
        lea     rsi, [rcx + 8*rdi]
.LBB0_2:
        sub     rsi, rcx
        sar     rsi, 3
        mov     qword ptr [rax], rcx
        mov     qword ptr [rax + 8], rdx
        mov     qword ptr [rax + 16], rsi
        ret
