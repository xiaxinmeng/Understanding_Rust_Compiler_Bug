asm
00000000000004f0 <naked>:
 4f0:   48 89 7d f0             mov    %rdi,-0x10(%rbp)
 4f4:   48 8b 7d f0             mov    -0x10(%rbp),%rdi
 4f8:   48 89 7d f8             mov    %rdi,-0x8(%rbp)
 4fc:   c3                      retq
 4fd:   c3                      retq
 4fe:   66 90                   xchg   %ax,%ax

0000000000000500 <non_naked>:
 500:   55                      push   %rbp
 501:   48 89 e5                mov    %rsp,%rbp
 504:   48 89 7d f0             mov    %rdi,-0x10(%rbp)
 508:   48 8b 7d f0             mov    -0x10(%rbp),%rdi
 50c:   48 89 7d f8             mov    %rdi,-0x8(%rbp)
 510:   5d                      pop    %rbp
 511:   c3                      retq
