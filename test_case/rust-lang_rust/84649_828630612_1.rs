asm
alloc::raw_vec::finish_grow:
        push    r15
        push    r14
        push    rbx
        mov     r15, rsi
        mov     r14, rdi
        test    rdx, rdx
        je      .LBB0_10
        mov     rbx, rdx
        mov     rdi, qword ptr [rcx]
        test    rdi, rdi
        je      .LBB0_2
        mov     rsi, qword ptr [rcx + 8]
        test    rsi, rsi
        je      .LBB0_2
        mov     rdx, rbx
        mov     rcx, r15
        call    qword ptr [rip + __rust_realloc@GOTPCREL]
        mov     rcx, r15
        test    rax, rax
        jne     .LBB0_9
        jmp     .LBB0_8
.LBB0_2:
        test    r15, r15
        je      .LBB0_3
        mov     rdi, r15
        mov     rsi, rbx
        call    qword ptr [rip + __rust_alloc@GOTPCREL]
        mov     rcx, r15
        test    rax, rax
        je      .LBB0_8
.LBB0_9:
        mov     qword ptr [r14 + 8], rax
        xor     eax, eax
        mov     rbx, rcx
        jmp     .LBB0_11
.LBB0_10:
        mov     qword ptr [r14 + 8], r15
        mov     eax, 1
        xor     ebx, ebx
        jmp     .LBB0_11
.LBB0_3:
        xor     ecx, ecx
        mov     rax, rbx
        test    rax, rax
        jne     .LBB0_9
.LBB0_8:
        mov     qword ptr [r14 + 8], r15
        mov     eax, 1
.LBB0_11:
        mov     qword ptr [r14 + 16], rbx
        mov     qword ptr [r14], rax
        pop     rbx
        pop     r14
        pop     r15
        ret

alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle:
        push    r14
        push    rbx
        sub     rsp, 56
        inc     rsi
        je      .LBB1_8
        mov     r14, rdi
        mov     rcx, qword ptr [rdi + 8]
        lea     rax, [rcx + rcx]
        cmp     rax, rsi
        cmova   rsi, rax
        cmp     rsi, 4
        mov     eax, 4
        cmova   rax, rsi
        mov     edx, 8
        xor     ebx, ebx
        mul     rdx
        setno   bl
        shl     rbx, 3
        test    rcx, rcx
        je      .LBB1_2
        mov     rdx, qword ptr [r14]
        shl     rcx, 3
        mov     qword ptr [rsp + 8], rdx
        mov     qword ptr [rsp + 16], rcx
        mov     qword ptr [rsp + 24], 8
        jmp     .LBB1_4
.LBB1_2:
        mov     qword ptr [rsp + 8], 0
.LBB1_4:
        lea     rdi, [rsp + 32]
        lea     rcx, [rsp + 8]
        mov     rsi, rax
        mov     rdx, rbx
        call    alloc::raw_vec::finish_grow
        mov     rdi, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 48]
        cmp     qword ptr [rsp + 32], 1
        je      .LBB1_5
        mov     qword ptr [r14], rdi
        shr     rsi, 3
        mov     qword ptr [r14 + 8], rsi
        add     rsp, 56
        pop     rbx
        pop     r14
        ret
.LBB1_5:
        test    rsi, rsi
        jne     .LBB1_6
.LBB1_8:
        call    qword ptr [rip + alloc::raw_vec::capacity_overflow@GOTPCREL]
        ud2
.LBB1_6:
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        ud2

.LCPI2_0:
        .long   1000000
        .long   0
        .long   0
        .long   0
example::foo:
        push    r15
        push    r14
        push    rbx
        mov     r14, rdi
        mov     edi, 8000000
        mov     esi, 8
        call    qword ptr [rip + __rust_alloc@GOTPCREL]
        test    rax, rax
        je      .LBB2_8
        mov     qword ptr [r14], rax
        vmovaps xmm0, xmmword ptr [rip + .LCPI2_0]
        vmovups xmmword ptr [r14 + 8], xmm0
        xor     esi, esi
        xor     ebx, ebx
        jmp     .LBB2_2
.LBB2_5:
        mov     qword ptr [rax + 8*rsi], rbx
        inc     rsi
        mov     qword ptr [r14 + 16], rsi
.LBB2_6:
        mov     rbx, r15
        cmp     r15, 1000000
        je      .LBB2_7
.LBB2_2:
        lea     r15, [rbx + 1]
        test    bl, 1
        jne     .LBB2_6
        cmp     rsi, qword ptr [r14 + 8]
        jne     .LBB2_5
        mov     rdi, r14
        call    alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        mov     rax, qword ptr [r14]
        mov     rsi, qword ptr [r14 + 16]
        jmp     .LBB2_5
.LBB2_7:
        mov     rax, r14
        pop     rbx
        pop     r14
        pop     r15
        ret
.LBB2_8:
        mov     edi, 8000000
        mov     esi, 8
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        ud2
