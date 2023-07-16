asm
example::clone_into_orig:
        push    r15
        push    r14
        push    r12
        push    rbx
        push    rax
        mov     r14, rdx
        mov     r12, rsi
        mov     r15, rdi
        mov     rbx, qword ptr [rdx + 16]
        cmp     rbx, rsi
        jb      .LBB1_2
        mov     qword ptr [r14 + 16], r12
        mov     rbx, r12
.LBB1_2:
        test    rbx, rbx
        je      .LBB1_4
        mov     rdi, qword ptr [r14]
        lea     rdx, [4*rbx]
        mov     rsi, r15
        call    qword ptr [rip + memcpy@GOTPCREL]
.LBB1_4:
        lea     rsi, [r15 + 4*rbx]
        sub     r12, rbx
        mov     rdi, r14
        mov     rdx, r12
        add     rsp, 8
        pop     rbx
        pop     r12
        pop     r14
        pop     r15
        jmp     alloc::vec::Vec<T>::extend_from_slice

example::clone_into_split:
        push    r15
        push    r14
        push    rbx
        mov     r14, rdx
        mov     rbx, rsi
        mov     rsi, rdi
        mov     rdx, qword ptr [rdx + 16]
        cmp     rdx, rbx
        jb      .LBB2_2
        mov     qword ptr [r14 + 16], rbx
        mov     rdx, rbx
.LBB2_2:
        lea     r15, [rsi + 4*rdx]
        sub     rbx, rdx
        test    rdx, rdx
        je      .LBB2_4
        mov     rdi, qword ptr [r14]
        shl     rdx, 2
        call    qword ptr [rip + memcpy@GOTPCREL]
.LBB2_4:
        mov     rdi, r14
        mov     rsi, r15
        mov     rdx, rbx
        pop     rbx
        pop     r14
        pop     r15
        jmp     alloc::vec::Vec<T>::extend_from_slice
