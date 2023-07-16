llvm
  %2 = alloca [16 x i8], align 16
  call void asm "", "=*m,~{dirflag},~{fpsr},~{flags}"([16 x i8]* %2) #2, !srcloc !2
