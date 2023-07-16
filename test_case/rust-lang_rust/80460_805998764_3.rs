asm
example::clone_slice_old:
        ; ...
.LBB0_7:
        cmp     r12, rbx
        je      .LBB0_10
        mov     edi, 4
        mov     esi, 4
        call    r14
        test    rax, rax
        je      .LBB0_13
        mov     rcx, qword ptr [r15 + 8*rbx]
        mov     ecx, dword ptr [rcx]
        mov     dword ptr [rax], ecx
        mov     qword ptr [r13 + 8*rbx], rax
        lea     rax, [rbx + 1]
        mov     rbx, rax
        cmp     rbp, rax
        jne     .LBB0_7
        ; ...

example::clone_slice_new:
        ; ...
.LBB1_7:
        mov     edi, 4
        mov     esi, 4
        call    r13
        test    rax, rax
        je      .LBB1_12
        mov     rcx, qword ptr [r15 + 8*rbp]
        mov     ecx, dword ptr [rcx]
        mov     dword ptr [rax], ecx
        mov     qword ptr [rbx + 8*rbp], rax
        add     rbp, 1
        cmp     r12, rbp
        jne     .LBB1_7
        ; ...
