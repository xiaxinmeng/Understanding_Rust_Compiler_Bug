llvm
; a, c
%0 = load i64, i64* %p, align 8
tail call void %g()
%1 = load i64, i64* %p, align 8
%2 = xor i64 %1, %0
ret i64 %2
