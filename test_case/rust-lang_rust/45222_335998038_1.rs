
//-----------------------------------------
       │     0000000000007300 <triangle_exc>:
       │     triangle_exc():
       │       inc    %rdi
       │     ↓ je     8c
       │       cmp    $0x3,%rdi
       │     ↓ jbe    7b
       │       mov    %rdi,%rcx
       │       and    $0xfffffffffffffffc,%rcx
       │     ↓ je     7b
       │       lea    -0x4(%rcx),%rax
       │       mov    %eax,%esi
       │       shr    $0x2,%esi
       │       inc    %esi
       │       and    $0x3,%rsi
       │     ↓ je     8f
       │       neg    %rsi
       │       mov    $0x1,%edx
       │       movq   %rdx,%xmm0
       │       pslldq $0x8,%xmm0
       │       pxor   %xmm2,%xmm2
       │       xor    %edx,%edx
       │       movdqa _fini+0x44,%xmm3
       │       movdqa _fini+0x54,%xmm4
       │       pxor   %xmm1,%xmm1
       │       data16 nopw %cs:0x0(%rax,%rax,1)
       │ 60:   paddq  %xmm0,%xmm2
       │       paddq  %xmm0,%xmm1
       │       paddq  %xmm3,%xmm1
       │       add    $0x4,%rdx
       │       paddq  %xmm4,%xmm0
       │       inc    %rsi
       │     ↑ jne    60
       │ 7b:   xor    %eax,%eax
       │       xor    %ecx,%ecx
       │       nop
       │ 80:   add    %rcx,%rax
       │       inc    %rcx
       │       cmp    %rdi,%rcx
       │     ↑ jb     80
       │ 8b: ← retq
       │ 8c:   xor    %eax,%eax
       │     ← retq
       │ 8f:   xor    %edx,%edx
       │       mov    $0x1,%esi
       │       movq   %rsi,%xmm0
       │       pslldq $0x8,%xmm0
       │       pxor   %xmm2,%xmm2
       │       pxor   %xmm1,%xmm1
       │ a8:   cmp    $0xc,%rax
       │       movdqa %xmm2,%xmm6
       │     ↓ jb     106
       │       mov    %rcx,%rax
       │       sub    %rdx,%rax
       │       movdqa _fini+0x64,%xmm3
       │       movdqa _fini+0x74,%xmm4
       │       movdqa _fini+0x84,%xmm5
       │ d0:   paddq  %xmm0,%xmm2
       │       paddq  %xmm0,%xmm1
 20.00 │       movdqa %xmm0,%xmm6
 10.00 │       paddq  %xmm6,%xmm6
       │       paddq  %xmm6,%xmm1
 10.00 │       paddq  %xmm2,%xmm6
 30.00 │       paddq  %xmm0,%xmm6
       │       paddq  %xmm0,%xmm1
 10.00 │       paddq  %xmm3,%xmm6
       │       paddq  %xmm4,%xmm1
       │       paddq  %xmm5,%xmm0
 20.00 │       movdqa %xmm6,%xmm2
       │     ↑ jne    d0
       │106:   paddq  %xmm1,%xmm6
       │       pshufd $0x4e,%xmm6,%xmm0
       │       paddq  %xmm6,%xmm0
       │       movq   %xmm0,%rax
       │       cmp    %rcx,%rdi
       │     ↑ jne    80
       │     ↑ jmpq   8b
