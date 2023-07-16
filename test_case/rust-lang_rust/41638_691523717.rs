asm
example::test:
        push    rbx
        cmp     rsi, 3
        jae     .LBB0_2
        xor     eax, eax
        pop     rbx
        ret
.LBB0_2:
        lea     rax, [rsi - 1]
        lea     rcx, [rsi - 3]
        xor     edx, edx
        cmp     rax, 2
        cmovae  rdx, rcx
        cmp     rdx, rsi
        jae     .LBB0_12
        xor     eax, eax
        lea     r10, [rip + .L__unnamed_1]
        lea     r9, [rip + .L__unnamed_2]
        lea     r8, [rip + .L__unnamed_3]
        jmp     .LBB0_4
.LBB0_10:
        add     rax, r11
        inc     rdi
        cmp     rsi, 3
        jb      .LBB0_11
.LBB0_4:
        dec     rsi
        mov     r11d, 1
        cmp     r10, rdi
        je      .LBB0_10
        movzx   edx, word ptr [rdi]
        xor     edx, 16724
        movzx   ebx, byte ptr [rdi + 2]
        xor     ebx, 71
        xor     ecx, ecx
        or      bx, dx
        setne   dl
        cmp     r9, rdi
        je      .LBB0_10
        mov     cl, dl
        test    ecx, ecx
        je      .LBB0_10
        movzx   edx, word ptr [rdi]
        xor     edx, 18260
        movzx   ebx, byte ptr [rdi + 2]
        xor     ebx, 65
        xor     ecx, ecx
        or      bx, dx
        setne   dl
        cmp     r8, rdi
        je      .LBB0_10
        mov     cl, dl
        test    ecx, ecx
        je      .LBB0_10
        movzx   ecx, word ptr [rdi]
        xor     ecx, 16724
        movzx   edx, byte ptr [rdi + 2]
        xor     edx, 65
        xor     r11d, r11d
        or      dx, cx
        sete    r11b
        jmp     .LBB0_10
.LBB0_11:
        pop     rbx
        ret
.LBB0_12:
        lea     rdx, [rip + .L__unnamed_4]
        mov     edi, 1
        xor     esi, esi
        call    qword ptr [rip + core::slice::slice_start_index_len_fail@GOTPCREL]
        ud2

.L__unnamed_5:
        .ascii  "/rustc/99111606fcda4fdb0646e4f7ee0f6cbcb76fb84a/library/core/src/slice/mod.rs"

.L__unnamed_4:
        .quad   .L__unnamed_5
        .asciz  "M\000\000\000\000\000\000\000\032\024\000\000\027\000\000"

.L__unnamed_1:
        .ascii  "TAG"

.L__unnamed_2:
        .ascii  "TGA"

.L__unnamed_3:
        .ascii  "TAA"
