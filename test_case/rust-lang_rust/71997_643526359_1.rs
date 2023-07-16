asm
example::copy_t:
        push    rax
        cmp     rsi, rcx
        mov     r8, rsi
        cmova   r8, rcx
        test    r8, r8
        je      .LBB2_5
        xor     r9d, r9d
.LBB2_2:
        cmp     rcx, r9
        je      .LBB2_6
        cmp     rsi, r9
        je      .LBB2_7
        movzx   eax, byte ptr [rdx + r9]
        mov     byte ptr [rdi + r9], al
        add     r9, 1
        cmp     r9, r8
        jb      .LBB2_2
.LBB2_5:
        pop     rax
        ret
.LBB2_6:
        lea     rdx, [rip + .L__unnamed_1]
        mov     rdi, rcx
        mov     rsi, rcx
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        ud2
.LBB2_7:
        lea     rdx, [rip + .L__unnamed_2]
        mov     rdi, rsi
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        ud2
