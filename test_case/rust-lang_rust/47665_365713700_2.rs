asm
  0.63 │14092:   lea    (%rdi,%rcx,4),%rbp
  0.01 │14096:   nopw   %cs:0x0(%rax,%rax,1)
 25.16 │140a0:   cmpq   $0x0,(%rsi,%rcx,8)
  7.68 │140a5: ↓ jne    140b6 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x176>
 28.34 │140a7:   add    $0x1,%rcx
  0.21 │140ab:   add    $0x4,%rbp
       │140af:   cmp    %rdx,%rcx
  2.63 │140b2: ↑ jb     140a0 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x160>
  0.04 │140b4: ↓ jmp    140ce <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x18e>
 24.54 │140b6:   test   %rbp,%rbp
  0.00 │140b9: ↓ je     140ce <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x18e>
  0.32 │140bb:   add    $0x1,%rcx
  5.10 │140bf:   mov    0x0(%rbp),%ebp
  3.31 │140c2:   add    %rbp,%rax
       │140c5:   cmp    %rdx,%rcx
  1.40 │140c8: ↑ jb     14092 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x152>
