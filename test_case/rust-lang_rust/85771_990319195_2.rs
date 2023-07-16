asm
angelicos::print_if_some:
    push    rax
    mov     eax, edi
    test    al, al
    jne     .LBB2_3
    test    al, 1
    jne     .LBB2_4
    and     eax, -65536
    cmp     eax, 2752512
    jne     .LBB2_3
    mov     edi, 42
    pop     rax
    jmp     qword, ptr, [rip, +, do_not_elim@GOTPCREL]
.LBB2_3:
    pop     rax
    ret
.LBB2_4:
    mov     byte, ptr, [rsp, +, 7], ah
    lea     rdi, [rip, +, .L__unnamed_1]
    lea     rcx, [rip, +, .L__unnamed_2]
    lea     r8, [rip, +, .L__unnamed_3]
    lea     rdx, [rsp, +, 7]
    mov     esi, 43
    call    qword, ptr, [rip, +, _ZN4core6result13unwrap_failed17h32ef6b3156e8fc57E@GOTPCREL]
    ud2 
