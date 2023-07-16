asm
example::noop:
        mov     rax, qword ptr [rdi + 16]
        test    rax, rax
        je      .LBB0_2
        mov     qword ptr [rdi + 16], rax
.LBB0_2:
        ret
