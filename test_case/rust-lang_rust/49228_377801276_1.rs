asm
<alloc::vec::Vec<T> as alloc::vec::SpecExtend<&'a T, core::slice::Iter<'a, T>>>::spec_extend:
  push rbp
  mov rbp, rsp
  push r15
  push r14
  push r13
  push r12
  push rbx
  sub rsp, 88
  mov r15, rdx
  mov r14, rsi
  mov r13, rdi
  sub r15, r14
  mov rbx, r15
  shr rbx, 2
  mov rsi, qword ptr [r13 + 8]
  mov r12, qword ptr [r13 + 16]
  mov rax, rsi
  sub rax, r12
  cmp rax, rbx
  jae .LBB0_14
  add r12, rbx
  jb .LBB0_8
  lea rax, [rsi + rsi]
  cmp r12, rax
  cmovb r12, rax
  mov ecx, 4
  mov rax, r12
  mul rcx
  jo .LBB0_8
  test rsi, rsi
  je .LBB0_9
  shl rsi, 2
  mov rdi, qword ptr [r13]
  lea r9, [rbp - 64]
  mov edx, 4
  mov r8d, 4
  mov rcx, rax
  call __rust_realloc@PLT
  test rax, rax
  jne .LBB0_5
  mov rax, qword ptr [rbp - 64]
  jmp .LBB0_12
.LBB0_9:
  lea rdx, [rbp - 64]
  mov esi, 4
  mov rdi, rax
  call __rust_alloc@PLT
  test rax, rax
  je .LBB0_12
.LBB0_5:
  xor ecx, ecx
  movdqa xmm0, xmmword ptr [rbp - 80]
  movdqa xmmword ptr [rbp - 96], xmm0
  test rcx, rcx
  je .LBB0_13
.LBB0_6:
  mov qword ptr [rbp - 64], rax
  movaps xmm0, xmmword ptr [rbp - 96]
  movups xmmword ptr [rbp - 56], xmm0
  lea rdi, [rbp - 128]
  lea rsi, [rbp - 64]
  call <core::heap::CollectionAllocErr as core::convert::From<core::heap::AllocErr>>::from@PLT
  movdqa xmm0, xmmword ptr [rbp - 128]
  movq rax, xmm0
  cmp rax, 3
  je .LBB0_14
  cmp rax, 2
  jne .LBB0_15
.LBB0_8:
  lea rdi, [rip + .Lbyte_str.5]
  call core::panicking::panic@PLT
  ud2
.LBB0_12:
  movups xmm0, xmmword ptr [rbp - 56]
  movaps xmmword ptr [rbp - 80], xmm0
  mov ecx, 1
  movdqa xmm0, xmmword ptr [rbp - 80]
  movdqa xmmword ptr [rbp - 96], xmm0
  test rcx, rcx
  jne .LBB0_6
.LBB0_13:
  mov qword ptr [r13], rax
  mov qword ptr [r13 + 8], r12
.LBB0_14:
  mov rdi, qword ptr [r13 + 16]
  add rbx, rdi
  mov qword ptr [r13 + 16], rbx
  shl rdi, 2
  add rdi, qword ptr [r13]
  and r15, -4
  mov rsi, r14
  mov rdx, r15
  call memcpy@PLT
  add rsp, 88
  pop rbx
  pop r12
  pop r13
  pop r14
  pop r15
  pop rbp
  ret
.LBB0_15:
  mov rax, qword ptr [rbp - 112]
  movdqa xmmword ptr [rbp - 64], xmm0
  mov qword ptr [rbp - 48], rax
  lea rdi, [rbp - 64]
  call <alloc::heap::Heap as core::heap::Alloc>::oom
  ud2

<alloc::heap::Heap as core::heap::Alloc>::oom:
  push rbp
  mov rbp, rsp
  call __rust_oom@PLT
  ud2

example::square:
  push rbp
  mov rbp, rsp
  lea rdx, [rsi + 4*rdx]
  pop rbp
  jmp <alloc::vec::Vec<T> as alloc::vec::SpecExtend<&'a T, core::slice::Iter<'a, T>>>::spec_extend

example::square2:
  push rbp
  mov rbp, rsp
  lea rdx, [rsi + 4*rdx]
  pop rbp
  jmp <alloc::vec::Vec<T> as alloc::vec::SpecExtend<&'a T, core::slice::Iter<'a, T>>>::spec_extend

.Lbyte_str.3:
  .ascii "capacity overflow"

.Lbyte_str.4:
  .ascii "liballoc/raw_vec.rs"

.Lbyte_str.5:
  .quad .Lbyte_str.3
  .asciz "\021\000\000\000\000\000\000"
  .quad .Lbyte_str.4
  .asciz "\023\000\000\000\000\000\000\000,\002\000\000&\000\000"
