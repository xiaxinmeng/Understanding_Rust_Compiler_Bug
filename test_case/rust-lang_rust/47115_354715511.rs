assembly
.LBB4_24:
  cmp r11, 4
  mov eax, 4
  cmovb rax, r11
  test rbx, rbx
  je .LBB4_18
  cmp rbx, 4
  mov edx, 4
  cmovb rdx, rbx
  test r13, r13
  je .LBB4_18
  mov qword ptr [rbp - 96], rax
  mov qword ptr [rbp - 48], rsi
  mov qword ptr [rbp - 56], r9
  cmp r11, 3
  jbe .LBB4_27
  mov qword ptr [rbp - 96], rdx
  mov qword ptr [rbp - 48], rsi
  mov qword ptr [rbp - 56], r9
  cmp rbx, 3
  jbe .LBB4_29
  cmp rax, 1
  je .LBB4_39
  lea r10, [r13 + rax]
  sub r11, rax
  lea r12, [r15 + rdx]
  sub rbx, rdx
  cmp rax, 3
  jb .LBB4_41
  je .LBB4_42
  movzx r14d, byte ptr [r13]
  movzx r8d, byte ptr [r13 + 1]
  movzx eax, byte ptr [r13 + 2]
  imul r13d, eax, 19595
  imul edi, r8d, 38470
  imul eax, r14d, 7471
  add eax, edi
  add eax, r13d
  shr eax, 16
  mov byte ptr [r15], al
  cmp rdx, 1
  je .LBB4_44
  mov byte ptr [r15 + 1], al
  cmp rdx, 3
  jb .LBB4_45
  mov byte ptr [r15 + 2], al
  je .LBB4_46
  mov byte ptr [r15 + 3], 0
  test r11, r11
  mov r15, r12
  mov r13, r10
  jne .LBB4_24
