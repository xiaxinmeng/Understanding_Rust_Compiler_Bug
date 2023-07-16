asm
example::use_ref_add:
        mov     rax, rdi
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdx]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        ret

example::Vec4::ref_add:
        movups  xmm0, xmmword ptr [rdi]
        movups  xmm1, xmmword ptr [rsi]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdx], xmm1
        ret

<example::Vec4 as core::ops::arith::Add>::add:
        mov     rax, rdi
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdx]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        ret

<example::Vec4 as core::ops::arith::AddAssign>::add_assign:
        movups  xmm0, xmmword ptr [rsi]
        movups  xmm1, xmmword ptr [rdi]
        addps   xmm1, xmm0
        movups  xmmword ptr [rdi], xmm1
        ret
