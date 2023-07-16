asm
  push r14
  push rbx
  sub rsp, 56
  mov r14, rdi
  lea rdx, [rsp + 8]
  mov edi, 512
  mov esi, 1
  call __rust_alloc@PLT
  mov rbx, rax
  test rbx, rbx
  je .LBB2_1
  mov rdi, rbx
  call Foo::new
  mov qword ptr [r14], rbx
  mov qword ptr [r14 + 8], 1
  mov qword ptr [r14 + 16], 1
  mov rax, r14
  add rsp, 56
  pop rbx
  pop r14
  ret
