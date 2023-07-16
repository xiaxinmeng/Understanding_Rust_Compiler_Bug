llvm
; playground::foo_add
; Function Attrs: nounwind nonlazybind readnone uwtable
define i32 @_ZN10playground7foo_add17h08377b0d50993157E(i32 %a, i32 %b) unnamed_addr #0 {
start:
  %0 = tail call i32 @llvm.sadd.sat.i32(i32 %a, i32 %b) #3
  ret i32 %0
}

; playground::bar_add
; Function Attrs: nounwind nonlazybind readnone uwtable
define i32 @_ZN10playground7bar_add17hf5ce7166079e2d09E(i32 %a, i32 %b) unnamed_addr #0 {
start:
  %0 = tail call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %a, i32 %b) #3
  %1 = extractvalue { i32, i1 } %0, 0
  %2 = extractvalue { i32, i1 } %0, 1
  %default..i = select i1 %2, i32 2147483647, i32 %1
  ret i32 %default..i
}
