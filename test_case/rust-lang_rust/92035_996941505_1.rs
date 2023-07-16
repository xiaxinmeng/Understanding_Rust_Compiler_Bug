asm
example::foo:
        push    rbp
        push    rbx
        push    rax
        lea     rax, [rdi - 1]
        xor     ebp, ebp
        cmp     rax, 999
        ja      .LBB0_3
        mov     rbx, rdi
        shl     rbx, 2
        mov     esi, 4
        mov     rdi, rbx
        call    qword ptr [rip + __rust_alloc_zeroed@GOTPCREL]
        test    rax, rax
        je      .LBB0_4
        mov     ebp, dword ptr [rax]
        mov     edx, 4
        mov     rdi, rax
        mov     rsi, rbx
        call    qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB0_3:
        mov     eax, ebp
        add     rsp, 8
        pop     rbx
        pop     rbp
        ret
.LBB0_4:
        mov     esi, 4
        mov     rdi, rbx
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        ud2
