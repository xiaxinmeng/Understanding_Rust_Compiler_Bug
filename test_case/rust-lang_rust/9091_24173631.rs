 asm
00000000000009f0 <_ZN4cast9transmute19hb053f0de10ad2e82an4v0.0E>:
 9f0:   64 48 3b 24 25 70 00    cmp    %fs:0x70,%rsp
 9f7:   00 00 
 9f9:   77 1a                   ja     a15 <_ZN4cast9transmute19hb053f0de10ad2e82an4v0.0E+0x25>
 9fb:   49 ba 08 00 00 00 00    movabs $0x8,%r10
 a02:   00 00 00 
 a05:   49 bb 00 00 00 00 00    movabs $0x0,%r11
 a0c:   00 00 00 
 a0f:   e8 70 00 00 00          callq  a84 <__morestack>
 a14:   c3                      retq   
 a15:   55                      push   %rbp
 a16:   48 89 e5                mov    %rsp,%rbp
 a19:   48 89 f0                mov    %rsi,%rax
 a1c:   5d                      pop    %rbp
 a1d:   c3                      retq   
 a1e:   90                      nop
 a1f:   90                      nop

0000000000000a20 <_ZN9fast_copy16hcf3bdb59c72c85b4v0.0E>:
 a20:   64 48 3b 24 25 70 00    cmp    %fs:0x70,%rsp
 a27:   00 00 
 a29:   77 1a                   ja     a45 <_ZN9fast_copy16hcf3bdb59c72c85b4v0.0E+0x25>
 a2b:   49 ba 28 00 00 00 00    movabs $0x28,%r10
 a32:   00 00 00 
 a35:   49 bb 00 00 00 00 00    movabs $0x0,%r11
 a3c:   00 00 00 
 a3f:   e8 40 00 00 00          callq  a84 <__morestack>
 a44:   c3                      retq   
 a45:   55                      push   %rbp
 a46:   48 89 e5                mov    %rsp,%rbp
 a49:   41 57                   push   %r15
 a4b:   41 56                   push   %r14
 a4d:   53                      push   %rbx
 a4e:   50                      push   %rax
 a4f:   4c 8b 36                mov    (%rsi),%r14
 a52:   4c 8b 7e 08             mov    0x8(%rsi),%r15
 a56:   48 8b 32                mov    (%rdx),%rsi
 a59:   48 8b 5a 08             mov    0x8(%rdx),%rbx
 a5d:   e8 8e ff ff ff          callq  9f0 <_ZN4cast9transmute19hb053f0de10ad2e82an4v0.0E>
 a62:   49 39 df                cmp    %rbx,%r15
 a65:   49 0f 42 df             cmovb  %r15,%rbx
 a69:   4c 89 f7                mov    %r14,%rdi
 a6c:   48 89 c6                mov    %rax,%rsi
 a6f:   48 89 da                mov    %rbx,%rdx
 a72:   e8 f9 fd ff ff          callq  870 <memmove@plt>
 a77:   48 83 c4 08             add    $0x8,%rsp
 a7b:   5b                      pop    %rbx
 a7c:   41 5e                   pop    %r14
 a7e:   41 5f                   pop    %r15
 a80:   5d                      pop    %rbp
 a81:   c3                      retq   
 a82:   66 90                   xchg   %ax,%ax
