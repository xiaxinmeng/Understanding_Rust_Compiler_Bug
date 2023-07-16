 llvm
; Function Attrs: noinline nounwind readnone uwtable
define internal fastcc zeroext i1 @_ZN3foo20hc5c0400b49933fc9daaE(i16 %in) unnamed_addr #0 {
match_case5.i:
  %coerce = alloca i16
  store i16 %in, i16* %coerce
  %coerce.p = bitcast i16* %coerce to %"enum.core::option::Option<[u8]>[#3]"*
  %0 = load %"enum.core::option::Option<[u8]>[#3]"* %coerce.p
  %.fca.0.extract4 = extractvalue %"enum.core::option::Option<[u8]>[#3]" %0, 0
  %.fca.2.0.extract6 = extractvalue %"enum.core::option::Option<[u8]>[#3]" %0, 2, 0
  %cond.i = icmp eq i8 %.fca.0.extract4, 1
  %1 = icmp eq i8 %.fca.2.0.extract6, 2
  %sret_slot.0.i = and i1 %cond.i, %1
  ret i1 %sret_slot.0.i
}
