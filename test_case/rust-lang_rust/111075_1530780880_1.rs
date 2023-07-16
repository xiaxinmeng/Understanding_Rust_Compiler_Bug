assembly
example::Deserializer::deserialize_bytes_2:
        push    rbp
        mov     rbp, rsp
        mov     rax, rdi
        mov     rcx, qword ptr [rsi + 16]
        cmp     rcx, -3
        ja      LBB0_2
        lea     rdx, [rcx + 2]
        cmp     rdx, qword ptr [rsi + 8]
        ja      LBB0_2
        mov     rdi, qword ptr [rsi]
        mov     qword ptr [rsi + 16], rdx
        movzx   ecx, word ptr [rdi + rcx]
        mov     word ptr [rax], cx
        mov     qword ptr [rax + 8], 0
        pop     rbp
        ret
LBB0_2:
        mov     qword ptr [rax], 0
        mov     qword ptr [rax + 8], 1
        mov     qword ptr [rax + 16], 0
        pop     rbp
        ret
