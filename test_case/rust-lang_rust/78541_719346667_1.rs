asm
get_two_mut:
    cmp     rdx, rcx
    je      .LBB7_4
    cmp     rdx, rsi
    jae     .LBB7_4
    cmp     rcx, rsi
    jae     .LBB7_4
    lea     rax, [rdi + 4*rdx]
    lea     rdx, [rdi + 4*rcx]
    ret
.LBB7_4:
    push    rax
    call    std::panicking::begin_panic
    ud2
