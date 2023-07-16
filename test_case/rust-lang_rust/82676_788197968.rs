asm
core::num::dec2flt::algorithm::power_of_ten::hf417b425dc5ff8a8:
  0xd839f00 <+0>:  pushq  %rax
  0xd839f01 <+1>:  movl   %edi, %eax
  0xd839f03 <+3>:  movswl %ax, %ecx
  0xd839f06 <+6>:  cmpl   $0xfffffecf, %ecx
  0xd839f0c <+12>: jl     0xd839f36              ; <+54> at algorithm.rs:17:5
  0xd839f0e <+14>: addl   $0x131, %eax
  0xd839f13 <+19>: movswq %ax, %rdi
  0xd839f17 <+23>: movzwl %ax, %eax
  0xd839f1a <+26>: cmpl   $0x263, %eax
  0xd839f1f <+31>: jae    0xd839f51              ; <+81> at algorithm.rs:19:15
  0xd839f21 <+33>: leaq   0x83bc830(%rip), %rcx
  0xd839f28 <+40>: movq   (%rcx,%rdi,8), %rax
  0xd839f2c <+44>: movzwl 0x1318(%rcx,%rdi,2), %edx
  0xd839f34 <+52>: popq   %rcx
  0xd839f35 <+53>: retq
  0xd839f36 <+54>: leaq   0x83bc7cc(%rip), %rdi
  0xd839f3d <+61>: leaq   0x65ab04c(%rip), %rdx
  0xd839f44 <+68>: movl   $0x23, %esi
  0xd839f49 <+73>: callq  *0x65ae839(%rip)
  0xd839f4f <+79>: ud2
  0xd839f51 <+81>: leaq   0x65ab050(%rip), %rdx
  0xd839f58 <+88>: movl   $0x263, %esi
  0xd839f5d <+93>: callq  *0x65ae81d(%rip)
  0xd839f63 <+99>: ud2
