asm
example::do_something:
        test    edi, edi
        je      .LBB0_2
        mov     eax, 104729
        xor     edx, edx
        div     edi
        test    edx, edx
        sete    al
        ret
.LBB0_2:
        push    rbp
        mov     rbp, rsp
        lea     rdi, [rip + panic_loc.7]
        call    core::panicking::panic@PLT

example::check:
        push    rbp
        mov     rbp, rsp
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        push    rax
        mov     rbx, rsi
        mov     r14, rdi
        lea     r13, [rbx + 4*rdx]
        shl     rdx, 2
        sar     rdx, 2
        xor     r12d, r12d
        cmp     rdx, 4
        jb      .LBB1_7
.LBB1_1:
        mov     r15, rbx
        mov     edi, dword ptr [r15]
        call    example::do_something
        test    al, al
        jne     .LBB1_17
        mov     edi, dword ptr [r15 + 4]
        call    example::do_something
        test    al, al
        jne     .LBB1_14
        mov     edi, dword ptr [r15 + 8]
        call    example::do_something
        test    al, al
        jne     .LBB1_15
        mov     edi, dword ptr [r15 + 12]
        call    example::do_something
        test    al, al
        jne     .LBB1_16
        add     r12, 4
        lea     rbx, [r15 + 16]
        mov     rax, r13
        sub     rax, rbx
        mov     rcx, rax
        sar     rcx, 63
        shr     rcx, 62
        add     rcx, rax
        sar     rcx, 2
        cmp     rcx, 3
        ja      .LBB1_1
        add     r15, 16
        mov     rbx, r15
.LBB1_7:
        cmp     rbx, r13
        je      .LBB1_11
.LBB1_8:
        mov     r15, r12
        mov     edi, dword ptr [rbx]
        call    example::do_something
        test    al, al
        jne     .LBB1_12
        add     rbx, 4
        lea     r12, [r15 + 1]
        cmp     r13, rbx
        jne     .LBB1_8
        xor     eax, eax
        mov     r12, r15
        jmp     .LBB1_18
.LBB1_11:
        xor     eax, eax
        jmp     .LBB1_18
.LBB1_12:
        mov     r12, r15
        jmp     .LBB1_17
.LBB1_14:
        or      r12, 1
        jmp     .LBB1_17
.LBB1_15:
        or      r12, 2
        jmp     .LBB1_17
.LBB1_16:
        or      r12, 3
.LBB1_17:
        mov     eax, 1
.LBB1_18:
        mov     qword ptr [r14], rax
        mov     qword ptr [r14 + 8], r12
        mov     rax, r14
        add     rsp, 8
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        pop     rbp
        ret

str.5:
        .ascii  "/tmp/compiler-explorer-compiler117520-5-qof3g7.ngo5vobt9/example.rs"

str.6:
        .ascii  "attempt to calculate the remainder with a divisor of zero"

panic_loc.7:
        .quad   str.6
        .quad   57
        .quad   str.5
        .quad   67
        .long   3
        .zero   4
