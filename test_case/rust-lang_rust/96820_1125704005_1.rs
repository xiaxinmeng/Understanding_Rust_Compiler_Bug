
        .text
        .intel_syntax noprefix
        .file   "example.6cc7c99e-cgu.0"
        .section        .text.t1,"ax",@progbits
        .globl  t1
        .p2align        4, 0x90
        .type   t1,@function
t1:
        mov     rax, qword ptr [rdi + 8]
        mov     eax, dword ptr [rax]
        ret
.Lfunc_end0:
        .size   t1, .Lfunc_end0-t1

        .section        .text.t2,"ax",@progbits
        .globl  t2
        .p2align        4, 0x90
        .type   t2,@function
t2:
        mov     rax, qword ptr [rdi]
        mov     eax, dword ptr [rax]
        ret
.Lfunc_end1:
        .size   t2, .Lfunc_end1-t2

        .section        ".note.GNU-stack","",@progbits
