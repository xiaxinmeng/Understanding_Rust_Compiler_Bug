 asm
  0,05 │4e0:   cmp    %rdx,%rax
       │     ↓ jae    50e
  0,03 │4e5:   mov    (%rbx),%rbp
  0,20 │       cmp    %rbp,%rax
  0,05 │     ↓ jae    527
 28,57 │4ed:   mov    (%r8),%rbp
  0,03 │       mov    (%rdi),%r9
  0,04 │       movsd  (%r9,%rax,8),%xmm1
  0,30 │       mulsd  0x0(%rbp,%rax,8),%xmm1
 40,50 │       lea    0x1(%rax),%rax
  0,06 │       addsd  %xmm1,%xmm0
 29,15 │       cmp    %r12,%rax
       │     ↑ jb     4e0
