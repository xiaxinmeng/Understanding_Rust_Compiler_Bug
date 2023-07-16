asm
 19.74 │14fa0:   cmpq   $0x0,(%r12,%rbx,8)
  6.54 │14fa5: ↓ jne    14fb2 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x1a2>
 21.06 │14fa7:   add    $0x1,%rbx
       │14fab:   cmp    %rbp,%rbx
  4.04 │14fae: ↑ jb     14fa0 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x190>
  0.04 │14fb0: ↓ jmp    14fde <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x1ce>
 28.36 │14fb2:   mov    0x0(%r13,%rbx,4),%eax
  4.34 │14fb7:   movq   $0x1,0x58(%rsp)
  5.45 │14fc0:   mov    %rax,0x60(%rsp)
  0.84 │14fc5:   mov    %r14,%rdi
  3.80 │14fc8: → callq  16380 <<u64 as core::iter::traits::Sum>::sum>
  2.52 │14fcd:   add    %rax,%r15
  2.07 │14fd0:   add    $0x1,%rbx
       │14fd4:   cmp    %rbp,%rbx
  0.26 │14fd7: ↑ jb     14fa0 <rayon::iter::plumbing::bridge_unindexed_producer_consumer+0x190>
