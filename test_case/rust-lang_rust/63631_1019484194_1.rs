asm
expect:
        push    r14
        push    rbx
        push    rax
        movabs  rax, 81985529216486895
        mov     qword ptr [rsp], rax
        mov     r14, qword ptr [rip + show@GOTPCREL]
        mov     rbx, rsp
        mov     rdi, rbx
        call    r14
        mov     rdi, rbx
        call    r14
        mov     rdi, rbx
        call    r14
        add     rsp, 8
        pop     rbx
        pop     r14
        ret
