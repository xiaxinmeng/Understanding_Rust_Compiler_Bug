assembly
example::Deserializer::deserialize_bytes_2:
        push    rbp
        mov     rbp, rsp
        push    rbx
        push    rax
        mov     rbx, rdi
        mov     rax, qword ptr [rsi + 16]
        cmp     rax, -3
        ja      LBB2_2
        lea     rcx, [rax + 2]
        cmp     rcx, qword ptr [rsi + 8]
        ja      LBB2_2
        mov     rdx, qword ptr [rsi]
        mov     qword ptr [rsi + 16], rcx
        movzx   eax, word ptr [rdx + rax]
        mov     word ptr [rbx], ax
        mov     qword ptr [rbx + 8], 0
LBB2_4:
        mov     rax, rbx
        add     rsp, 8
        pop     rbx
        pop     rbp
        ret
LBB2_2:
        mov     rdi, rbx
        call    example::Deserializer::deserialize_bytes_error
        jmp     LBB2_4
