asm
example::copy_slice_old:
        ; ...
        mov     rdi, rax
        mov     rax, r13
        shr     rax, 2
        test    rdi, rdi
        cmove   rax, rdi
        test    rdi, rdi
        je      .LBB0_7
        ; ...

example::copy_slice_new:
        ; ...
        mov     rbp, rax
        test    rbp, rbp
        je      .LBB1_7
        ; ...
