 asm
       │1 600:   mov    %rsi,%rcx
 21,23 │         add    $0x1,%rcx
  0,03 │         cmovb  %r12,%rcx
  0,02 │         cmp    %rsi,%rdx
  0,03 │       ↓ jbe    638
 20,87 │1 610:   mov    0x0(%rbp),%rbx
  0,10 │         cmp    %rsi,%rbx
       │       ↓ jbe    646
  0,03 │1 619:   mov    (%r9),%rbx
  0,01 │         mov    (%rdi),%r15
 21,45 │         movsd  (%r15,%rsi,8),%xmm1
  0,21 │         mulsd  (%rbx,%rsi,8),%xmm1
  7,03 │         addsd  %xmm1,%xmm0
 28,06 │         cmp    %r12,%rcx
  0,02 │         mov    %rcx,%rsi
       │       ↑ jb     600
