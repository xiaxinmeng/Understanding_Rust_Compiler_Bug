
       │    0000000000007430 <triangle_inc>:
       │    triangle_inc():
       │      xor    %r8d,%r8d
       │      mov    $0xffffffffffffffff,%r9
       │      xor    %r10d,%r10d
       │      xor    %eax,%eax
       │    ↓ jmp    29
       │      data16 data16 data16 data16 data16 nopw %cs:0x0(%rax,%rax,1)
       │20:   add    %r10,%rax
  1.79 │      mov    %rdx,%rdi
  2.98 │      mov    %rsi,%r10
 17.86 │29:   cmp    %rdi,%r10
       │      mov    $0x1,%ecx
  7.14 │      cmovb  %r9,%rcx
 16.07 │      cmove  %r8,%rcx
  3.57 │      test   %rcx,%rcx
  1.79 │      mov    $0x0,%edx
  1.79 │      mov    $0x1,%esi
 14.29 │    ↑ je     20
       │      cmp    $0xffffffffffffffff,%rcx
       │    ↓ jne    57
  2.98 │      lea    0x1(%r10),%rsi
  3.57 │      mov    %rdi,%rdx
 26.19 │    ↑ jmp    20
       │57: ← retq
