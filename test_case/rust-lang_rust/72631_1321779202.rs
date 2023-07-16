asm
example::extend_one:
        push    rbx
        mov     rbx, rdi
        mov     rsi, qword ptr [rdi + 16]
        cmp     rsi, qword ptr [rdi + 8]
        jne     .LBB3_2
        mov     rdi, rbx
        call    alloc::raw_vec::RawVec<T,A>::reserve_for_push
        mov     rsi, qword ptr [rbx + 16]
.LBB3_2:
        mov     rax, qword ptr [rbx]
        mov     byte ptr [rax + rsi], 0
        inc     rsi
        mov     qword ptr [rbx + 16], rsi
        pop     rbx
        ret

example::extend_some:
        push    rbx
        mov     rbx, rdi
        mov     rsi, qword ptr [rdi + 16]
        cmp     qword ptr [rdi + 8], rsi
        je      .LBB4_1
.LBB4_2:
        mov     rax, qword ptr [rbx]
        mov     byte ptr [rax + rsi], 0
        inc     rsi
        mov     qword ptr [rbx + 16], rsi
        pop     rbx
        ret
.LBB4_1:
        mov     edx, 1
        mov     rdi, rbx
        call    alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        mov     rsi, qword ptr [rbx + 16]
        jmp     .LBB4_2
