asm
example::rotate_u8_right1:
        push    rbp
        push    r14
        push    rbx
        test    rsi, rsi
        je      .LBB0_11
        mov     rdx, rsi
        add     rdx, -1
        je      .LBB0_10
        mov     r14, rdi
        cmp     rsi, 23
        ja      .LBB0_8
        mov     bpl, byte ptr [r14]
        mov     eax, 1
        mov     ecx, 1
        sub     rcx, rsi
        xor     esi, esi
        mov     edi, 1
        jmp     .LBB0_4
.LBB0_5:
        add     rdi, 1
.LBB0_4:
        mov     ebx, ebp
        movzx   ebp, byte ptr [r14 + rdi]
        mov     byte ptr [r14 + rdi], bl
        cmp     rdi, rdx
        jb      .LBB0_5
        add     rdi, rcx
        je      .LBB0_9
        cmp     rdi, rax
        cmovb   rax, rsi
        cmovb   rdi, rsi
        jmp     .LBB0_4
.LBB0_8:
        lea     rdi, [r14 + 1]
        mov     bpl, byte ptr [r14 + rdx]
        mov     rsi, r14
        call    qword ptr [rip + memmove@GOTPCREL]
.LBB0_9:
        mov     byte ptr [r14], bpl
.LBB0_10:
        pop     rbx
        pop     r14
        pop     rbp
        ret
.LBB0_11:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 34
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
