
.LCPI0_0:
        .quad   900
        .quad   900
example::foo:
        push    rbx
        mov     rbx, rdi
        mov     edi, 900
        mov     esi, 1
        call    qword ptr [rip + __rust_alloc_zeroed@GOTPCREL]
        test    rax, rax
        je      .LBB0_1
        mov     qword ptr [rbx], rax
        movaps  xmm0, xmmword ptr [rip + .LCPI0_0]
        movups  xmmword ptr [rbx + 8], xmm0
        mov     rax, rbx
        pop     rbx
        ret
.LBB0_1:
        mov     edi, 900
        mov     esi, 1
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        ud2
