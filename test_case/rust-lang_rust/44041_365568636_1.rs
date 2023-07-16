asm
example::mytest:
  cmp edi, esi
  je .LBB0_1
  setge al
  add rdx, rdx
  movzx eax, al
  sub rdx, rax
  add rdx, 2
  mov rax, rdx
  ret
.LBB0_1:
  push rbp
  mov rbp, rsp
  xor edi, edi
  call std::process::exit@PLT
  ud2
