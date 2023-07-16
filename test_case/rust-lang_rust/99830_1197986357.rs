
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN60_$LT$mul_bug..Mod$u20$as$u20$core..ops..arith..MulAssign$GT$10mul_assign17h1004f3da66b04a20E"(i32* noalias noundef align 4 dereferenceable(4) %0, i32 %1) unnamed_addr #1 {
  %3 = load i32, i32* %0, align 4
  %4 = call { i32, i32 } undef(i32 1000000009, i32 %1, i32 %3) #20, !srcloc !15
  %5 = extractvalue { i32, i32 } %4, 1
  store i32 %5, i32* %0, align 4
  br label %6

6:                                                ; preds = %2
  ret void
}
