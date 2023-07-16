asm
example::g:
        cmp     qword ptr [rdi], 1
        jne     .LBB0_1
        mov     rax, qword ptr [rdi + 8]
        ret
.LBB0_1:
        push    rbp
        mov     rbp, rsp
        lea     rdi, [rip + .Lbyte_str.2]
        call    core::panicking::panic@PLT
        ud2

.Lbyte_str.0:
        .ascii  "called `Option::unwrap()` on a `None` value"

.Lbyte_str.1:
        .ascii  "libcore/option.rs"

.Lbyte_str.2:
        .quad   .Lbyte_str.0
        .asciz  "+\000\000\000\000\000\000"
        .quad   .Lbyte_str.1
        .asciz  "\021\000\000\000\000\000\000\000O\001\000\000\025\000\000"
