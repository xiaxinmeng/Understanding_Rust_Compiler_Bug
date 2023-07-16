asm
    vcmpeqps    %xmm0, %xmm7, %xmm0
    movdq2q     %xmm0, %mm0
    vxorps      %xmm0, %xmm0, %xmm0
    pmovmskb    %mm0, %eax
