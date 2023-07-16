asm
example::bar:
  push rbx
  mov eax, 4096
  call __rust_probestack
  sub rsp, rax
  mov rax, rdi
  mov rdi, rsp
  mov edx, 4096
  mov rsi, rax
  call memcpy@PLT
  mov edi, 4096
  mov esi, 1
  call __rust_alloc@PLT
  mov rbx, rax
  test rbx, rbx
  je .LBB2_1
  mov rsi, rsp
  mov edx, 4096
  mov rdi, rbx
  call memcpy@PLT
  mov rax, rbx
  add rsp, 4096
  pop rbx
  ret
.LBB2_1:
  call <alloc::alloc::Global as core::alloc::GlobalAlloc>::oom
  ud2
