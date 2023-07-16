asm
example::bar:
  sub rsp, 520
  lea rdi, [rsp + 8]
  call qword ptr [rip + example::BigTest::new@GOTPCREL]
  add rsp, 520
  ret

.LCPI1_0:
  .long 123
  .long 123
  .long 123
  .long 123
example::BigTest::new:
  push rbx
  sub rsp, 512
  mov rbx, rdi
  movaps xmm0, xmmword ptr [rip + .LCPI1_0]
  movaps xmmword ptr [rsp], xmm0
  movaps xmmword ptr [rsp + 16], xmm0
  movaps xmmword ptr [rsp + 32], xmm0
  movaps xmmword ptr [rsp + 48], xmm0
  movaps xmmword ptr [rsp + 64], xmm0
  movaps xmmword ptr [rsp + 80], xmm0
  movaps xmmword ptr [rsp + 96], xmm0
  movaps xmmword ptr [rsp + 112], xmm0
  movaps xmmword ptr [rsp + 128], xmm0
  movaps xmmword ptr [rsp + 144], xmm0
  movaps xmmword ptr [rsp + 160], xmm0
  movaps xmmword ptr [rsp + 176], xmm0
  movaps xmmword ptr [rsp + 192], xmm0
  movaps xmmword ptr [rsp + 208], xmm0
  movaps xmmword ptr [rsp + 224], xmm0
  movaps xmmword ptr [rsp + 240], xmm0
  movaps xmmword ptr [rsp + 256], xmm0
  movaps xmmword ptr [rsp + 272], xmm0
  movaps xmmword ptr [rsp + 288], xmm0
  movaps xmmword ptr [rsp + 304], xmm0
  movaps xmmword ptr [rsp + 320], xmm0
  movaps xmmword ptr [rsp + 336], xmm0
  movaps xmmword ptr [rsp + 352], xmm0
  movaps xmmword ptr [rsp + 368], xmm0
  movaps xmmword ptr [rsp + 384], xmm0
  movaps xmmword ptr [rsp + 400], xmm0
  movaps xmmword ptr [rsp + 416], xmm0
  movaps xmmword ptr [rsp + 432], xmm0
  movaps xmmword ptr [rsp + 448], xmm0
  movaps xmmword ptr [rsp + 464], xmm0
  movaps xmmword ptr [rsp + 480], xmm0
  movaps xmmword ptr [rsp + 496], xmm0
  mov rsi, rsp
  mov edx, 512
  call qword ptr [rip + memcpy@GOTPCREL]
  mov rax, rbx
  add rsp, 512
  pop rbx
  ret
