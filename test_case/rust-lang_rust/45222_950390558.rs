asm
example::triangle_exc:
        cmp     rdi, -1
        je      .LBB0_1
        lea     rcx, [rdi - 1]
        mov     rax, rdi
        mul     rcx
        shld    rdx, rax, 63
        add     rdx, rdi
        mov     rax, rdx
        ret
.LBB0_1:
        xor     edx, edx
        mov     rax, rdx
        ret

example::triangle_inc:
        xor     eax, eax
        xor     ecx, ecx
.LBB1_1:
        mov     rdx, rcx
        cmp     rcx, rdi
        adc     rcx, 0
        add     rax, rdx
        cmp     rdx, rdi
        jae     .LBB1_3
        cmp     rcx, rdi
        jbe     .LBB1_1
.LBB1_3:
        ret
