asm
example::mytest:
  xor eax, eax
  xor ecx, ecx
  cmp edi, esi
  setge cl
  lea rcx, [rcx + rcx - 1]
  cmove rcx, rax
  cmp rcx, 1
  je .LBB0_3
  test rcx, rcx
  je .LBB0_2
  lea rax, [rax + 2*rdx]
  add rax, 2
  ret
.LBB0_3:
  mov rax, -1
  lea rax, [rax + 2*rdx]
  add rax, 2
  ret
.LBB0_2:
  push rbp
  mov rbp, rsp
  xor edi, edi
  call std::process::exit@PLT
  ud2
