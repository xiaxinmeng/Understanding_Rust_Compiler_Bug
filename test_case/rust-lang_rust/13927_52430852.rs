 llvm
define internal fastcc zeroext i1 @_ZN3foo20hc5c0400b49933fc9daaE(i32 %in) unnamed_addr #0 {
match_case5.i:
  %0 = and i32 %in, -65281
  %1 = icmp eq i32 %0, 131073
  ret i1 %1
}
