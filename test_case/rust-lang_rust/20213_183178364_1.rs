 llvm
%2 = call double asm "fldln2; fldl $1; fyl2x",
"={st},*m,~{dirflag},~{fpsr},~{flags}"(double* %1) #2, !srcloc !1
