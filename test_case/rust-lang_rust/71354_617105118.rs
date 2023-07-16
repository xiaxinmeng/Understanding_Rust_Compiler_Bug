asm
example::foo:
        push    rax
        mov     rcx, qword ptr [rdi + 16]
        cmp     rcx, 3
        jb      .LBB5_2
        mov     rdx, qword ptr [rdi]
        mov     rax, qword ptr [rdx + 8*rcx - 8]
        lea     rsi, [rcx - 2]
        mov     qword ptr [rdi + 16], rsi
        add     rax, qword ptr [rdx + 8*rcx - 16]
        pop     rcx
        ret
.LBB5_2:
        call    std::panicking::begin_panic
        ud2
