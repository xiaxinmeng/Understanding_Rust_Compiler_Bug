 llvm
define internal fastcc zeroext i1 @_ZN3foo20he087cfc23527c69deaaE(%"enum.core::option::Option<[\22u16\22]>[#2]") unnamed_addr #0 {
match_case5.i:
  %.fca.0.extract4 = extractvalue %"enum.core::option::Option<[\22u16\22]>[#2]" %0, 0
  %.fca.2.0.extract6 = extractvalue %"enum.core::option::Option<[\22u16\22]>[#2]" %0, 2, 0
  %cond.i = icmp eq i16 %.fca.0.extract4, 1
  %1 = icmp eq i16 %.fca.2.0.extract6, 2
  %sret_slot.0.i = and i1 %cond.i, %1
  ret i1 %sret_slot.0.i
}
