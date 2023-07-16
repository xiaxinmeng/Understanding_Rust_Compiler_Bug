asm
example::rorate_left_with_reverse:
        push    rbx
        mov     r8, rsi
        sub     r8, rdx
        jb      .LBB1_26
        cmp     rdx, 2
        jb      .LBB1_9
        mov     r9, rdx
        shr     r9
        cmp     r9, 1
        jne     .LBB1_4
        xor     ecx, ecx
        test    dl, 2
        jne     .LBB1_8
        jmp     .LBB1_9
.LBB1_4:
        lea     rax, [rdi + 4]
        lea     r10, [rdi + 4*rdx]
        add     r10, -4
        and     r9, -2
        neg     r9
        xor     ecx, ecx
.LBB1_5:
        mov     r11d, dword ptr [rax - 4]
        mov     ebx, dword ptr [r10 + 4*rcx]
        mov     dword ptr [rax - 4], ebx
        mov     dword ptr [r10 + 4*rcx], r11d
        mov     r11d, dword ptr [rax]
        mov     ebx, dword ptr [r10 + 4*rcx - 4]
        mov     dword ptr [rax], ebx
        mov     dword ptr [r10 + 4*rcx - 4], r11d
        add     rax, 8
        add     rcx, -2
        cmp     r9, rcx
        jne     .LBB1_5
        neg     rcx
        test    dl, 2
        je      .LBB1_9
.LBB1_8:
        mov     rax, rcx
        not     rax
        add     rax, rdx
        mov     r9d, dword ptr [rdi + 4*rcx]
        mov     ebx, dword ptr [rdi + 4*rax]
        mov     dword ptr [rdi + 4*rcx], ebx
        mov     dword ptr [rdi + 4*rax], r9d
.LBB1_9:
        cmp     r8, 2
        jb      .LBB1_17
        mov     r9, r8
        shr     r9
        cmp     r9, 1
        jne     .LBB1_12
        xor     ecx, ecx
        test    r8b, 2
        jne     .LBB1_16
        jmp     .LBB1_17
.LBB1_12:
        lea     rax, [rdi + 4*rdx]
        add     rax, 4
        lea     r10, [rdi + 4*rsi]
        add     r10, -4
        and     r9, -2
        neg     r9
        xor     ecx, ecx
.LBB1_13:
        mov     r11d, dword ptr [rax - 4]
        mov     ebx, dword ptr [r10 + 4*rcx]
        mov     dword ptr [rax - 4], ebx
        mov     dword ptr [r10 + 4*rcx], r11d
        mov     r11d, dword ptr [rax]
        mov     ebx, dword ptr [r10 + 4*rcx - 4]
        mov     dword ptr [rax], ebx
        mov     dword ptr [r10 + 4*rcx - 4], r11d
        add     rax, 8
        add     rcx, -2
        cmp     r9, rcx
        jne     .LBB1_13
        neg     rcx
        test    r8b, 2
        je      .LBB1_17
.LBB1_16:
        lea     rax, [rdi + 4*rdx]
        mov     rdx, rcx
        not     rdx
        add     r8, rdx
        mov     edx, dword ptr [rax + 4*rcx]
        mov     ebx, dword ptr [rax + 4*r8]
        mov     dword ptr [rax + 4*rcx], ebx
        mov     dword ptr [rax + 4*r8], edx
.LBB1_17:
        cmp     rsi, 2
        jb      .LBB1_25
        mov     r8, rsi
        shr     r8
        cmp     r8, 1
        jne     .LBB1_20
        xor     eax, eax
        test    sil, 2
        jne     .LBB1_24
        jmp     .LBB1_25
.LBB1_20:
        lea     rdx, [rdi + 4]
        lea     rcx, [rdi + 4*rsi]
        add     rcx, -4
        and     r8, -2
        neg     r8
        xor     eax, eax
.LBB1_21:
        mov     r9d, dword ptr [rdx - 4]
        mov     ebx, dword ptr [rcx + 4*rax]
        mov     dword ptr [rdx - 4], ebx
        mov     dword ptr [rcx + 4*rax], r9d
        mov     r9d, dword ptr [rdx]
        mov     ebx, dword ptr [rcx + 4*rax - 4]
        mov     dword ptr [rdx], ebx
        mov     dword ptr [rcx + 4*rax - 4], r9d
        add     rdx, 8
        add     rax, -2
        cmp     r8, rax
        jne     .LBB1_21
        neg     rax
        test    sil, 2
        je      .LBB1_25
.LBB1_24:
        mov     rcx, rax
        not     rcx
        add     rcx, rsi
        mov     edx, dword ptr [rdi + 4*rax]
        mov     esi, dword ptr [rdi + 4*rcx]
        mov     dword ptr [rdi + 4*rax], esi
        mov     dword ptr [rdi + 4*rcx], edx
.LBB1_25:
        pop     rbx
        ret
.LBB1_26:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 32
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
