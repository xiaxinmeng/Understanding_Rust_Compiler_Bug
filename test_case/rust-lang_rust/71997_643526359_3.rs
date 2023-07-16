asm
example::copy_s:
        test    rcx, rcx
        je      .LBB1_2
        push    rax
        mov     rsi, rdx
        mov     rdx, rcx
        call    qword ptr [rip + memcpy@GOTPCREL]
        add     rsp, 8
.LBB1_2:
        ret
