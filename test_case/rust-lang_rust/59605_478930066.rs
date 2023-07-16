
       │ 80:┌─→cmpq   $0x1,-0x8(%rbx)
 30.06 │    │↓ jne    b0
       │    │  lea    0x30(%rsp),%rdi
       │    │  mov    %rbx,%rsi
       │    │→ callq  *0x68a7b(%rip)        # 74ff0 <<alloc::string::String as core::clone::Clone>::clone>
       │    │  mov    0x30(%rsp),%rax
       │    │  lea    0x38(%rsp),%rcx
       │    │  movups (%rcx),%xmm0
       │    │  movaps %xmm0,(%rsp)
       │    │  mov    $0x1,%ecx
       │    │↓ jmp    b5
       │    │  nop
  0.24 │ b0:│  mov    (%rbx),%rax
       │    │  xor    %ecx,%ecx
       │ b5:│  mov    %rcx,0x50(%rsp)
 25.21 │    │  mov    %rax,0x58(%rsp)
 26.02 │    │  movaps (%rsp),%xmm0
  6.58 │    │  movups %xmm0,0x0(%r13)
  4.89 │    │  movups 0x0(%r13),%xmm0
  6.69 │    │  movaps %xmm0,(%rsp)
  0.09 │    │  test   %rcx,%rcx
       │    │↓ jne    100
       │    │  add    $0x20,%rbx
       │    │  mov    $0x1,%r12d
       │    │  mov    %rax,%rbp
       │    ├──add    $0xffffffffffffffe0,%r15
       │    └──jne    80
