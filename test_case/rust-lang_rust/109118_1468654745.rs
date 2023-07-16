asm
example::main:
        push    r15
        push    r14
        push    r12
        push    rbx
        sub     rsp, 88
        mov     dword ptr [rsp + 12], 1061273700 ; value of a
        mov     dword ptr [rsp + 16], 1060095721 ; precalculated value of b
        lea     rax, [rsp + 16]
        mov     qword ptr [rsp + 24], rax
        mov     r12, qword ptr [rip + core::fmt::num::<impl core::fmt::LowerHex for u32>::fmt@GOTPCREL]
        mov     qword ptr [rsp + 32], r12
        lea     r15, [rip + .L__unnamed_1]
        mov     qword ptr [rsp + 56], r15
        mov     qword ptr [rsp + 64], 2
        mov     qword ptr [rsp + 40], 0
        lea     rbx, [rsp + 24]
        mov     qword ptr [rsp + 72], rbx
        mov     qword ptr [rsp + 80], 1
        mov     r14, qword ptr [rip + std::io::stdio::_print@GOTPCREL]
        lea     rdi, [rsp + 40]
        call    r14
        movss   xmm0, dword ptr [rsp + 12]
        call    qword ptr [rip + sinf@GOTPCREL] ; value of c calculated here
        movss   dword ptr [rsp + 20], xmm0
        lea     rax, [rsp + 20]
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 32], r12
        mov     qword ptr [rsp + 56], r15
        mov     qword ptr [rsp + 64], 2
        mov     qword ptr [rsp + 40], 0
        mov     qword ptr [rsp + 72], rbx
        mov     qword ptr [rsp + 80], 1
        lea     rdi, [rsp + 40]
        call    r14
        lea     rax, [rsp + 12]
        mov     qword ptr [rsp + 24], rax
        mov     rax, qword ptr [rip + core::fmt::float::<impl core::fmt::Display for f32>::fmt@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 56], r15
        mov     qword ptr [rsp + 64], 2
        mov     qword ptr [rsp + 40], 0
        mov     qword ptr [rsp + 72], rbx
        mov     qword ptr [rsp + 80], 1
        lea     rdi, [rsp + 40]
        call    r14
        add     rsp, 88
        pop     rbx
        pop     r12
        pop     r14
        pop     r15
        ret

.L__unnamed_2:

.L__unnamed_3:
        .byte   10

.L__unnamed_1:
        .quad   .L__unnamed_2
        .zero   8
        .quad   .L__unnamed_3
        .asciz  "\001\000\000\000\000\000\000"
