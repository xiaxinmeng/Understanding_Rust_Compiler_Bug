asm
do_ants(): # @do_ants()
  sub rsp, 216
  mov qword ptr [rsp], 0
  xorps xmm0, xmm0
  movups xmmword ptr [rsp + 198], xmm0
  movups xmmword ptr [rsp + 184], xmm0
  mov rdi, rsp
  call do_item(Outer&)
  add rsp, 216
  ret
