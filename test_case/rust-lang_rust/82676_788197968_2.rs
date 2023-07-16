asm
core::num::dec2flt::algorithm::power_of_ten::hf417b425dc5ff8a8:
  0xd839ec0 <+0>:  pushq  %rax
  0xd839ec1 <+1>:  movl   %edi, %eax
  0xd839ec3 <+3>:  movswl %ax, %ecx
  0xd839ec6 <+6>:  cmpl   $0xfffffecf, %ecx
  0xd839ecc <+12>: jl     0xd839ef6              ; <+54> at algorithm.rs:17:5
  0xd839ece <+14>: addl   $0x131, %eax
  0xd839ed3 <+19>: movswq %ax, %rdi
  0xd839ed7 <+23>: movzwl %ax, %eax
  0xd839eda <+26>: cmpl   $0x263, %eax
  0xd839edf <+31>: jae    0xd839f11              ; <+81> at algorithm.rs:19:15
  0xd839ee1 <+33>: leaq   0x53409c0(%rip), %rcx  ; core::num::dec2flt::table::POWERS::hf668a1d7d9ba717f
  0xd839ee8 <+40>: movq   (%rcx,%rdi,8), %rax
  0xd839eec <+44>: movzwl 0x1318(%rcx,%rdi,2), %edx
  0xd839ef4 <+52>: popq   %rcx
  0xd839ef5 <+53>: retq
  0xd839ef6 <+54>: leaq   0x83bc80c(%rip), %rdi
  0xd839efd <+61>: leaq   0x65ab08c(%rip), %rdx
  0xd839f04 <+68>: movl   $0x23, %esi
  0xd839f09 <+73>: callq  *0x65ae879(%rip)
  0xd839f0f <+79>: ud2
  0xd839f11 <+81>: leaq   0x65ab090(%rip), %rdx
  0xd839f18 <+88>: movl   $0x263, %esi
  0xd839f1d <+93>: callq  *0x65ae85d(%rip)
  0xd839f23 <+99>: ud2
