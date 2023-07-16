asm
example::return_vec_via_const_item:
        mov     rax, rdi
        mov     rcx, qword ptr [rip + .L__unnamed_2]
        mov     qword ptr [rdi], rcx
        mov     rcx, qword ptr [rip + .L__unnamed_2+8]
        mov     qword ptr [rdi + 8], rcx
        mov     rcx, qword ptr [rip + .L__unnamed_2+16]
        mov     qword ptr [rdi + 16], rcx
        ret

.L__unnamed_2:
        .asciz  "\b\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000"
