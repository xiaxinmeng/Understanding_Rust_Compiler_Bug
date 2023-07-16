asm
<example::Blueprint as core::cmp::PartialEq>::eq:
        vmovdqu xmm0, xmmword ptr [rdi]
        vpcmpeqd        xmm0, xmm0, xmmword ptr [rsi]
        vmovmskps       eax, xmm0
        cmp     al, 15
        mov     eax, dword ptr [rdi + 16]
        sete    cl
        cmp     eax, dword ptr [rsi + 16]
        sete    al
        and     al, cl
        ret
