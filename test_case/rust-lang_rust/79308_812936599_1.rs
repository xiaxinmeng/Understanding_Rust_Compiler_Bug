asm
example::vec_cast:
        mov     rax, rdi
        mov     rdi, qword ptr [rsi]
        mov     r8, qword ptr [rsi + 8]
        mov     rsi, qword ptr [rsi + 16]
        mov     rcx, rdi
        test    rsi, rsi
        je      .LBB0_7
        lea     r9, [8*rsi - 8]
        mov     ecx, r9d
        shr     ecx, 3
        add     ecx, 1
        mov     rdx, rdi
        and     rcx, 7
        je      .LBB0_4
        neg     rcx
        mov     rdx, rdi
.LBB0_3:
        add     rdx, 8
        inc     rcx
        jne     .LBB0_3
.LBB0_4:
        lea     rcx, [rdi + 8*rsi]
        cmp     r9, 56
        jb      .LBB0_7
        lea     rsi, [rdi + 8*rsi]
        sub     rsi, rdx
.LBB0_6:
        add     rsi, -64
        jne     .LBB0_6
.LBB0_7:
        sub     rcx, rdi
        sar     rcx, 3
        mov     qword ptr [rax], rdi
        mov     qword ptr [rax + 8], r8
        mov     qword ptr [rax + 16], rcx
        ret
