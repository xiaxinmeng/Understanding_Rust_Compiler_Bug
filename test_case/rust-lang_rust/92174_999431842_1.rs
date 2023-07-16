asm
do_checks(std::vector<int, std::allocator<int> >):          # @do_checks(std::vector<int, std::allocator<int> >)
        push    rax
        mov     rax, qword ptr [rdi]
        mov     rcx, qword ptr [rdi + 8]
        mov     rdx, rcx
        sub     rdx, rax
        je      .LBB0_4
        sar     rdx, 2
        mov     rsi, -1
.LBB0_2:                                # =>This Inner Loop Header: Depth=1
        add     rsi, 1
        cmp     rdx, rsi
        je      .LBB0_5
        add     rax, 4
        cmp     rax, rcx
        jne     .LBB0_2
.LBB0_4:
        pop     rax
        ret
.LBB0_5:
        mov     edi, offset .L.str
        mov     rsi, rdx
        xor     eax, eax
        call    std::__throw_out_of_range_fmt(char const*, ...)
.L.str:
        .asciz  "vector::_M_range_check: __n (which is %zu) >= this->size() (which is %zu)"
