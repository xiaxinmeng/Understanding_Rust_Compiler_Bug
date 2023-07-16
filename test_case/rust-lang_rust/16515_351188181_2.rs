llvm
; c
%0 = load i64, i64* %p, align 8, !alias.scope !0
tail call void %g(), !noalias !0
%1 = load i64, i64* %p, align 8, !alias.scope !0
%2 = xor i64 %1, %0
ret i64 %2
