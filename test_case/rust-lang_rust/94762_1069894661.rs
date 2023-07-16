asm
000000000315c300 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj>:
 315c300:       55                      push   %rbp
 315c301:       41 57                   push   %r15
 315c303:       41 56                   push   %r14
 315c305:       41 55                   push   %r13
 315c307:       41 54                   push   %r12
 315c309:       53                      push   %rbx
 315c30a:       48 81 ec a8 06 00 00    sub    $0x6a8,%rsp
 315c311:       48 89 54 24 18          mov    %rdx,0x18(%rsp)
 315c316:       48 89 74 24 10          mov    %rsi,0x10(%rsp)
...
 315c902:       8b 0d 48 3b f8 01       mov    0x1f83b48(%rip),%ecx        # 50e0450 <_ZL13SmallLoopCost+0x80>
 315c908:       39 4c 24 18             cmp    %ecx,0x18(%rsp)
 315c90c:       0f 83 a6 01 00 00       jae    315cab8 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x7b8>
 315c912:       89 c8                   mov    %ecx,%eax
 315c914:       31 d2                   xor    %edx,%edx
 315c916:       f7 74 24 18             divl   0x18(%rsp)
