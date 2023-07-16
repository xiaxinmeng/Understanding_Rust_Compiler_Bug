asm
example::foo:
  push rax
  mov edi, 8
  mov esi, 1
  call __rust_alloc@PLT
  test rax, rax
  je .LBB1_1
  mov qword ptr [rax], 0
  pop rcx
  ret
.LBB1_1:
  call <alloc::alloc::Global as core::alloc::GlobalAlloc>::oom
  ud2

example::bar:
  sub rsp, 24
  mov byte ptr [rsp + 16], 0
  mov qword ptr [rsp + 8], 0
  mov edi, 9
  mov esi, 1
  call __rust_alloc@PLT
  test rax, rax
  je .LBB2_1
  mov cl, byte ptr [rsp + 16]
  mov byte ptr [rax + 8], cl
  mov rcx, qword ptr [rsp + 8]
  mov qword ptr [rax], rcx
  add rsp, 24
  ret
.LBB2_1:
  call <alloc::alloc::Global as core::alloc::GlobalAlloc>::oom
  ud2
