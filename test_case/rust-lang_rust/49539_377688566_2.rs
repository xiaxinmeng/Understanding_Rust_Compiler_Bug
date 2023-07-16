asm
  push r15
  push r14
  push rbx
  sub rsp, 528
  mov r14, rdi
  lea rdx, [rsp + 16]
  mov edi, 512
  mov esi, 1
  call __rust_alloc@PLT
  mov rbx, rax
  test rbx, rbx
  je .LBB3_1
  lea r15, [rsp + 16]
  mov rdi, r15
  call Foo::new
  mov edx, 512
  mov rdi, rbx
  mov rsi, r15
  call memcpy@PLT
  mov qword ptr [r14], rbx
  mov qword ptr [r14 + 8], 1
  mov qword ptr [r14 + 16], 1
  mov rax, r14
  add rsp, 528
  pop rbx
  pop r14
  pop r15
  ret
