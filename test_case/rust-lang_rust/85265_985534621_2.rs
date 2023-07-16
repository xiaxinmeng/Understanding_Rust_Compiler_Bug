asm
example::use_ref_add:
        movd    xmm0, esi
        mov     rax, rsi
        shld    rax, rdi, 32
        shr     rsi, 32
        movq    xmm1, rax
        movq    xmm2, rsi
        punpckldq       xmm2, xmm1
        movd    xmm1, edi
        movd    xmm3, edx
        addps   xmm3, xmm1
        movd    xmm1, ecx
        addss   xmm1, xmm0
        shrd    rdx, rcx, 32
        shr     rcx, 32
        movq    xmm0, rdx
        movq    xmm4, rcx
        punpckldq       xmm4, xmm0
        addps   xmm4, xmm2
        movd    eax, xmm3
        movd    ecx, xmm1
        movd    edx, xmm4
        shufps  xmm4, xmm4, 85
        movd    esi, xmm4
        shl     rsi, 32
        shl     rdx, 32
        or      rdx, rcx
        or      rax, rsi
        ret

example::Vec4::ref_add:
        movups  xmm0, xmmword ptr [rdi]
        movups  xmm1, xmmword ptr [rsi]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdx], xmm1
        ret

<example::Vec4 as core::ops::arith::Add>::add:
        movd    xmm0, esi
        mov     rax, rsi
        shld    rax, rdi, 32
        shr     rsi, 32
        movq    xmm1, rax
        movq    xmm2, rsi
        punpckldq       xmm2, xmm1
        movd    xmm1, edi
        movd    xmm3, edx
        addps   xmm3, xmm1
        movd    xmm1, ecx
        addss   xmm1, xmm0
        shrd    rdx, rcx, 32
        shr     rcx, 32
        movq    xmm0, rdx
        movq    xmm4, rcx
        punpckldq       xmm4, xmm0
        addps   xmm4, xmm2
        movd    eax, xmm3
        movd    ecx, xmm1
        movd    edx, xmm4
        shufps  xmm4, xmm4, 85
        movd    esi, xmm4
        shl     rsi, 32
        shl     rdx, 32
        or      rdx, rcx
        or      rax, rsi
        ret

<example::Vec4 as core::ops::arith::AddAssign>::add_assign:
        movq    xmm0, rdx
        movq    xmm1, rsi
        punpcklqdq      xmm1, xmm0
        movups  xmm0, xmmword ptr [rdi]
        addps   xmm0, xmm1
        movups  xmmword ptr [rdi], xmm0
        ret
