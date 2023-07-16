asm
check:
        push    rax
        cmp     rdi, -1
        je      .LBB0_1
        mov     rax, rdi
        pop     rcx
        ret
.LBB0_1:
        call    std::process::abort@PLT
        ud2
