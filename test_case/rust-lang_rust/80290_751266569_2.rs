
       │    000000000007ac00 <core::ptr::write>:
       │    _ZN4core3ptr5write17h8616c78d24e53297E():
 18,93 │      sub    $0x28,%rsp
  8,13 │      mov    %rsi,(%rsp)
 18,96 │      mov    %rdi,0x10(%rsp)
       │      movb   $0x0,0xf(%rsp)
  5,42 │      movb   $0x1,0xf(%rsp)
 10,72 │      mov    (%rsp),%rax
 29,71 │      mov    %rax,(%rdi)
  8,13 │      movb   $0x0,0xf(%rsp)
       │      add    $0x28,%rsp
       │    ← retq 
