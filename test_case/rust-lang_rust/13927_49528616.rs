 llvm
define zeroext i1 @_ZN3foo20h8f1b2f0c18884ca6daaE(%"enum.core::option::Option<[u8]>[#3]") unnamed_addr #0 {
match_case5.i:
  %.fca.0.extract7 = extractvalue %"enum.core::option::Option<[u8]>[#3]" %0, 0
  %.fca.2.0.extract9 = extractvalue %"enum.core::option::Option<[u8]>[#3]" %0, 2, 0
  %cond.i = icmp eq i8 %.fca.0.extract7, 1
  %1 = icmp eq i8 %.fca.2.0.extract9, 2
  %__make_return_pointer.0.i = and i1 %cond.i, %1
  ret i1 %__make_return_pointer.0.i
}
