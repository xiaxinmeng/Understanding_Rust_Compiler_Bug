 llvm
define dereferenceable(1) i8* @_ZN8get_bool20h3a2efbe887530bffeaaE(%"enum.core::option::Option<[\22bool\22]>[#2]"* noalias dereferenceable(2)) unnamed_addr #0 {
entry-block:
  %1 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 0
  %2 = load i8* %1, align 1, !range !0, !alias.scope !1
  %3 = icmp eq i8 %2, 1
  br i1 %3, label %"_ZN6option15Option$LT$T$GT$6unwrap21h10756339029509221854E.exit", label %then-block-21-

then-block-21-:                                   ; preds = %entry-block
  %4 = bitcast %"enum.core::option::Option<[\22bool\22]>[#2]"* %0 to i16*
  store i16 257, i16* %4, align 2
  br label %"_ZN6option15Option$LT$T$GT$6unwrap21h10756339029509221854E.exit"

"_ZN6option15Option$LT$T$GT$6unwrap21h10756339029509221854E.exit": ; preds = %then-block-21-, %entry-block
  %5 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 2, i64 0
  ret i8* %5
}
