asm
example::copy_slice_old:
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        ; ...

example::copy_slice_new:
        push    rbp
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        sub     rsp, 8
        ; ...
