asm
    vcmpunordps   .LCPI187_26(%rip), %xmm3, %xmm4
.Ltmp9279:
    .loc       70 14 69
    movdq2q    %xmm4, %mm0
.Ltmp9280:
    .loc       52 2377 5
    pmovmskb   %mm0, %eax
