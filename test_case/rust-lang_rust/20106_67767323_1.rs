
  call void asm "\0A      mov $$60, %rax\0A      mov $0, %rdi\0A      syscall\0A      ", "r,~{rax},~{rdi},~{dirflag},~{fpsr},~{flags}"(i64 %0), !srcloc !0
