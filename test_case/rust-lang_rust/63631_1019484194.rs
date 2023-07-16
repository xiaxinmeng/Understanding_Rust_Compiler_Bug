asm
got:
        push    rbx
        sub     rsp, 32
        movabs  rax, 81985529216486895
        mov     qword ptr [rsp + 8], rax
        mov     rbx, qword ptr [rip + show@GOTPCREL]
        lea     rdi, [rsp + 8]
        call    rbx
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 16], rax
        lea     rdi, [rsp + 16]
        call    rbx
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 24], rax
        lea     rdi, [rsp + 24]
        call    rbx
        add     rsp, 32
        pop     rbx
        ret
