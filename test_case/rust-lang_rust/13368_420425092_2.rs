llvm
  %arr = alloca [16 x i8], align 1
  %0 = call [16 x i8] asm "", "=m,~{dirflag},~{fpsr},~{flags}"(), !srcloc !6
  store [16 x i8] %0, [16 x i8]* %arr, align 1
