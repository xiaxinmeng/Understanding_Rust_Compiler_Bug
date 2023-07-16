

define i32 @foo(i32 %a) unnamed_addr #0 !dbg !6 {
  %0 = sdiv i32 %a, 256, !dbg !10
  ret i32 %0, !dbg !11
}
