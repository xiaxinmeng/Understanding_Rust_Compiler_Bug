asm
b:
        mov     QWORD PTR [rdi], 0
        mov     QWORD PTR [rdi+8], 0
        mov     QWORD PTR [rdi+16], 0
        mov     QWORD PTR [rdi+24], 0
        mov     QWORD PTR [rdi+32], 0
        mov     QWORD PTR [rdi+40], 0
        mov     rax, rdi
        ret
c:
        mov     QWORD PTR [rdi], 0
        mov     QWORD PTR [rdi+8], 0
        mov     QWORD PTR [rdi+16], 0
        mov     QWORD PTR [rdi+24], 0
        mov     QWORD PTR [rdi+32], 0
        mov     QWORD PTR [rdi+40], 0
        mov     rax, rdi
        ret
