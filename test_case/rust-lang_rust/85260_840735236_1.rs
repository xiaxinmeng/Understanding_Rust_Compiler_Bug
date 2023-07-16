asm
example::if_else:
        cmp     rsi, rdx
        je      .LBB0_2
        cmovl   rcx, rdx
        mov     qword ptr [rdi], rcx
.LBB0_2:
        cmp     rsi, rdx
        sete    al
        ret

example::contains_match_:
        cmp     rsi, rdx
        jl      .LBB1_2
        mov     al, 1
        mov     rdx, rcx
        je      .LBB1_3
.LBB1_2:
        mov     qword ptr [rdi], rdx
        xor     eax, eax
.LBB1_3:
        ret

example::if_else_cmp:
        cmp     rsi, rdx
        jl      .LBB2_2
        mov     al, 1
        mov     rdx, rcx
        je      .LBB2_3
.LBB2_2:
        mov     qword ptr [rdi], rdx
        xor     eax, eax
.LBB2_3:
        ret
