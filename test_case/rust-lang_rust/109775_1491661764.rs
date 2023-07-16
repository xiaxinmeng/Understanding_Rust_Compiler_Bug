ll
define i128 @test() {
  %a = alloca i128
  store ptr null, ptr %a
  %v = load i128, ptr %a, !range !0
  ret i128 %v
}

!0 = !{i128 1, i128 0}
