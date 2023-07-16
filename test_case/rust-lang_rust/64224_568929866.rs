asm
example::bar:
  push r14
  push rbx
  sub rsp, 24
  mov ebx, 42
  call qword ptr [rip + foo@GOTPCREL]
.LBB2_6:
  mov eax, ebx
  add rsp, 24
  pop rbx
  pop r14
  ret
  mov rbx, qword ptr [rax + 256]
  mov r14, qword ptr [rax + 264]
  mov qword ptr [rax + 256], 0
  mov rdi, rax
  call qword ptr [rip + _Unwind_DeleteException@GOTPCREL]
  mov qword ptr [rsp + 8], rbx
  mov qword ptr [rsp + 16], r14
  test rbx, rbx
  je .LBB2_2
  mov edi, -1
  call qword ptr [rip + update_panic_count@GOTPCREL]
  mov ebx, 13
  jmp .LBB2_6
.LBB2_2:
  lea rdi, [rip + .L__unnamed_1]
  lea rdx, [rip + .L__unnamed_2]
  mov esi, 43
  call qword ptr [rip + core::panicking::panic@GOTPCREL]
  ud2
  mov rbx, rax
  lea rdi, [rsp + 8]
  call core::ptr::real_drop_in_place
  mov rdi, rbx
  call _Unwind_Resume@PLT
  ud2
